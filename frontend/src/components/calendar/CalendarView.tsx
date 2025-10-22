import { useState, useCallback, useEffect } from 'react';
import { Calendar as BigCalendar, dateFnsLocalizer } from 'react-big-calendar';
import { Views } from 'react-big-calendar';
import format from 'date-fns/format';
import parse from 'date-fns/parse';
import startOfWeek from 'date-fns/startOfWeek';
import getDay from 'date-fns/getDay';
import enUS from 'date-fns/locale/en-US';
import { id } from 'date-fns/locale';
import { dashboardApi, type BookingCalendar } from '@/api/dashboard';
import 'react-big-calendar/lib/css/react-big-calendar.css';

const locales = {
  'en-US': enUS,
  'id-ID': id,
};

const localizer = dateFnsLocalizer({
  format,
  parse,
  startOfWeek,
  getDay,
  locales,
});

interface CalendarViewProps {
  onBookingClick?: (booking: BookingCalendar) => void;
}

const CalendarView: React.FC<CalendarViewProps> = ({ onBookingClick }) => {
  const [bookings, setBookings] = useState<BookingCalendar[]>([]);
  const [loading, setLoading] = useState(false);
  const [currentDate, setCurrentDate] = useState(new Date());
  const [currentView, setCurrentView] = useState<string>(Views.MONTH);

  const fetchBookings = useCallback(async (start: Date, end: Date) => {
    try {
      setLoading(true);
      const calendarBookings = await dashboardApi.getCalendarBookings(
        start.toISOString(),
        end.toISOString()
      );
      setBookings(calendarBookings);
    } catch (error) {
      console.error('Failed to fetch calendar bookings:', error);
    } finally {
      setLoading(false);
    }
  }, []);

  // Initial fetch when component mounts
  useEffect(() => {
    handleNavigate(currentDate);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleNavigate = useCallback((date: Date) => {
    setCurrentDate(date);

    let start: Date;
    let end: Date;

    if (currentView === Views.MONTH) {
      start = new Date(date.getFullYear(), date.getMonth(), 1);
      end = new Date(date.getFullYear(), date.getMonth() + 1, 0);
    } else if (currentView === Views.WEEK) {
      const weekStart = startOfWeek(date, { weekStartsOn: 1 });
      start = weekStart;
      end = new Date(weekStart.getTime() + 7 * 24 * 60 * 60 * 1000);
    } else {
      start = new Date(date.getFullYear(), date.getMonth(), date.getDate());
      end = new Date(start.getTime() + 24 * 60 * 60 * 1000);
    }

    fetchBookings(start, end);
  }, [currentView, fetchBookings]);

  const handleViewChange = useCallback((view: string) => {
    setCurrentView(view);
    handleNavigate(currentDate);
  }, [currentDate, handleNavigate]);

  const handleSelectEvent = useCallback((event: any) => {
    if (onBookingClick && event.resource) {
      onBookingClick(event.resource);
    }
  }, [onBookingClick]);

  const updateBookingStatus = async (bookingId: string, status: string) => {
    try {
      await dashboardApi.updateBookingStatus(bookingId, status);
      handleNavigate(currentDate);
    } catch (error) {
      console.error('Failed to update booking status:', error);
    }
  };

  const CustomEvent = ({ event }: { event: any }) => {
    const booking = event.resource;

    const handleQuickAccept = async (e: React.MouseEvent) => {
      e.stopPropagation();
      try {
        await dashboardApi.updateBookingStatus(booking.id, 'confirmed');
        // Refresh calendar by calling navigate function
        window.location.reload();
      } catch (error) {
        console.error('Failed to accept booking:', error);
        alert('Gagal menerima booking. Silakan coba lagi.');
      }
    };

    const handleQuickReject = async (e: React.MouseEvent) => {
      e.stopPropagation();
      const reason = prompt('Alasan penolakan:');
      if (!reason || !reason.trim()) {
        alert('Alasan penolakan harus diisi.');
        return;
      }

      try {
        await dashboardApi.updateBookingStatus(booking.id, 'cancelled');
        // Refresh calendar by calling navigate function
        window.location.reload();
      } catch (error) {
        console.error('Failed to reject booking:', error);
        alert('Gagal menolak booking. Silakan coba lagi.');
      }
    };

    return (
      <div
        className="calendar-event group"
        style={{
          padding: '4px',
          fontSize: '11px',
          fontWeight: '500',
          cursor: 'pointer',
          borderRadius: '4px',
          height: '100%',
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden',
          position: 'relative',
        }}
        onClick={() => {
          console.log('Event clicked:', event);
          if (onBookingClick && event.resource) {
            onBookingClick(event.resource);
          }
        }}
        title={`${booking.customer_name} - ${booking.service_type}\nStatus: ${booking.status}\nKlik untuk detail`}
      >
        <div className="truncate font-medium" style={{ width: '100%' }}>
          {booking.customer_name}
        </div>
        <div className="truncate" style={{ width: '100%', fontSize: '10px', opacity: 0.9 }}>
          {booking.service_type}
        </div>

        {booking.status === 'pending' && (
          <div
            className="absolute top-0 right-0 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
            style={{ background: 'rgba(255,255,255,0.95)', borderRadius: '0 4px 0 4px' }}
          >
            <button
              onClick={handleQuickAccept}
              className="text-green-600 hover:text-green-700 p-1"
              title="Terima Booking"
              style={{ fontSize: '10px' }}
            >
              ✓
            </button>
            <button
              onClick={handleQuickReject}
              className="text-red-600 hover:text-red-700 p-1"
              title="Tolak Booking"
              style={{ fontSize: '10px' }}
            >
              ✗
            </button>
          </div>
        )}
      </div>
    );
  };

  const calendarEvents = bookings.map(booking => ({
    id: booking.id,
    title: `${booking.customer_name} - ${booking.service_type}`,
    start: new Date(booking.start_time),
    end: new Date(booking.end_time),
    resource: booking,
  }));

  const eventStyleGetter = (event: any) => {
    const booking = event.resource as BookingCalendar;
    let backgroundColor = '#3174ad';

    switch (booking.status) {
      case 'pending':
        backgroundColor = '#f59e0b';
        break;
      case 'confirmed':
        backgroundColor = '#10b981';
        break;
      case 'completed':
        backgroundColor = '#3b82f6';
        break;
      case 'cancelled':
        backgroundColor = '#ef4444';
        break;
      default:
        backgroundColor = '#6b7280';
    }

    return {
      style: {
        backgroundColor,
        borderRadius: '4px',
        border: 'none',
        color: 'white',
        cursor: 'pointer',
        padding: '2px 4px',
        fontSize: '12px',
        fontWeight: '500',
        boxShadow: '0 1px 2px rgba(0,0,0,0.1)',
        transition: 'all 0.2s ease',
      },
    };
  };

  return (
    <div className="bg-white rounded-lg shadow-sm p-6">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-semibold text-gray-900">Kalender Booking</h2>
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 text-sm">
            <div className="w-3 h-3 bg-yellow-500 rounded-full relative">
              <div className="absolute -top-1 -right-1 w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
            </div>
            <span className="text-gray-600 font-medium">Pending (Aksi Diperlukan)</span>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <div className="w-3 h-3 bg-green-500 rounded-full"></div>
            <span className="text-gray-600">Confirmed</span>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <div className="w-3 h-3 bg-blue-500 rounded-full"></div>
            <span className="text-gray-600">Completed</span>
          </div>
          <div className="flex items-center gap-2 text-sm">
            <div className="w-3 h-3 bg-red-500 rounded-full"></div>
            <span className="text-gray-600">Cancelled</span>
          </div>
        </div>
      </div>

      <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-6">
        <h3 className="font-medium text-blue-800 mb-2">📋 Cara Penggunaan</h3>
        <div className="text-sm text-blue-700 space-y-1">
          <p>• <strong>Klik booking</strong> untuk melihat detail dan mengubah status</p>
          <p>• <strong>Hover pada booking pending</strong> untuk tombol cepat ✓ (terima) atau ✗ (tolak)</p>
          <p>• Booking dengan <span className="text-yellow-600 font-semibold">titik merah berkedip</span> memerlukan konfirmasi segera</p>
        </div>
      </div>

      {loading ? (
        <div className="flex justify-center py-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-pink-600"></div>
        </div>
      ) : (
        <div style={{ height: 600 }}>
          <BigCalendar
            localizer={localizer}
            events={calendarEvents}
            startAccessor="start"
            endAccessor="end"
            onSelectEvent={handleSelectEvent}
            onView={handleViewChange}
            onNavigate={handleNavigate}
            view={currentView}
            date={currentDate}
            eventPropGetter={eventStyleGetter}
            components={{
              event: CustomEvent,
            }}
            messages={{
              next: "Selanjutnya",
              previous: "Sebelumnya",
              today: "Hari Ini",
              month: "Bulan",
              week: "Minggu",
              day: "Hari",
              agenda: "Agenda",
              date: "Tanggal",
              time: "Waktu",
              event: "Acara",
              noEventsInRange: "Tidak ada acara dalam rentang ini.",
            }}
          />
        </div>
      )}
    </div>
  );
};

export default CalendarView;