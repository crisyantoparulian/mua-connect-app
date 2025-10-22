import { useState, useEffect } from 'react';
import {
  format,
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  addDays,
  addMonths,
  subMonths,
  isSameMonth,
  isSameDay,
  isToday,
  parseISO
} from 'date-fns';
import { id } from 'date-fns/locale';
import {
  ChevronLeft,
  ChevronRight,
  Clock,
  Plus,
  X,
  User,
  MapPin,
  Phone,
  Loader2
} from 'lucide-react';
import { dashboardApi } from '../../api/dashboard';

// Define all required interfaces locally to avoid import issues
interface TimeSlot {
  id: string;
  start_time: string;
  end_time: string;
  is_available: boolean;
  recurring?: boolean;
  day_of_week?: number;
  specific_date?: string;
}

interface CreateAvailabilityRequest {
  start_time: string;
  end_time: string;
  recurring: boolean;
  day_of_week?: number[];
  specific_date?: string;
}

interface BookingCalendar {
  id: string;
  customer_name: string;
  service_type: string;
  start_time: string;
  end_time: string;
  status: 'pending' | 'confirmed' | 'completed' | 'cancelled';
  customer_phone?: string;
  location?: string;
  notes?: string;
}

// Use BookingCalendar from local definition instead of local Booking interface
type Booking = BookingCalendar;

interface AvailabilityCalendarProps {
  onBookingClick?: (booking: Booking) => void;
}

const AvailabilityCalendar: React.FC<AvailabilityCalendarProps> = ({ onBookingClick: _onBookingClick }) => {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);
  const [viewMode, setViewMode] = useState<'month' | 'week' | 'day'>('month');
  const [showAvailabilityModal, setShowAvailabilityModal] = useState(false);
  const [showBookingModal, setShowBookingModal] = useState(false);
  const [selectedBooking, setSelectedBooking] = useState<Booking | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);

  const [timeSlots, setTimeSlots] = useState<TimeSlot[]>([]);
  const [bookings, setBookings] = useState<Booking[]>([]);

  // Form state for availability
  const [availabilityForm, setAvailabilityForm] = useState({
    start_time: '09:00',
    end_time: '17:00',
    recurring: false,
    day_of_week: [] as number[]
  });

  // Load real data from API
  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        // Load availability slots
        const timeSlotsData = await dashboardApi.getAvailabilitySlots();
        console.log('Loaded time slots:', timeSlotsData);

        // If no availability slots exist, create some default ones for demonstration
        if (!timeSlotsData || timeSlotsData.length === 0) {
          console.log('No availability slots found, creating defaults...');
          try {
            // Create default weekday availability (Monday-Friday, 9am-5pm)
            const defaultSlot = await dashboardApi.createAvailabilitySlot({
              start_time: '09:00',
              end_time: '17:00',
              recurring: true,
              day_of_week: [1, 2, 3, 4, 5] // Monday-Friday
            });
            console.log('Created default slot:', defaultSlot);
            setTimeSlots([defaultSlot]);
          } catch (createError) {
            console.error('Failed to create default availability:', createError);
            setTimeSlots([]);
          }
        } else {
          setTimeSlots(timeSlotsData);
        }

        // Load calendar bookings for current month
        const monthStart = startOfMonth(currentDate);
        const monthEnd = endOfMonth(monthStart);
        const bookingsData = await dashboardApi.getCalendarBookings(
          monthStart.toISOString(),
          monthEnd.toISOString()
        );
        console.log('Loaded bookings:', bookingsData);
        setBookings(bookingsData || []);
      } catch (error) {
        console.error('Failed to load calendar data:', error);
        // Set empty arrays on error to prevent infinite loading
        setTimeSlots([]);
        setBookings([]);
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [currentDate]);

  const renderHeader = () => {
    const dateFormat = "MMMM yyyy";

    return (
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-4">
          <h2 className="text-2xl font-bold text-gray-900">
            {format(currentDate, dateFormat, { locale: id })}
          </h2>

          {/* View Mode Selector */}
          <div className="flex items-center bg-gray-100 rounded-lg p-1">
            {(['month', 'week', 'day'] as const).map((mode) => (
              <button
                key={mode}
                onClick={() => setViewMode(mode)}
                className={`px-3 py-1 rounded-md text-sm font-medium transition-colors ${
                  viewMode === mode
                    ? 'bg-white text-pink-600 shadow-sm'
                    : 'text-gray-600 hover:text-gray-900'
                }`}
              >
                {mode === 'month' ? 'Bulan' : mode === 'week' ? 'Minggu' : 'Hari'}
              </button>
            ))}
          </div>
        </div>

        <div className="flex items-center gap-2">
          {/* Navigation Buttons */}
          <button
            onClick={() => setCurrentDate(subMonths(currentDate, 1))}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ChevronLeft className="w-5 h-5" />
          </button>

          <button
            onClick={() => setCurrentDate(new Date())}
            className="px-3 py-1 text-sm bg-gray-100 hover:bg-gray-200 rounded-lg transition-colors"
          >
            Hari Ini
          </button>

          <button
            onClick={() => setCurrentDate(addMonths(currentDate, 1))}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <ChevronRight className="w-5 h-5" />
          </button>

          {/* Action Buttons */}
          <button
            onClick={() => setShowAvailabilityModal(true)}
            className="flex items-center gap-2 px-4 py-2 bg-pink-600 text-white rounded-lg hover:bg-pink-700 transition-colors"
          >
            <Plus className="w-4 h-4" />
            Atur Jadwal
          </button>
        </div>
      </div>
    );
  };

  const renderDaysOfWeek = () => {
    const days = ['Min', 'Sen', 'Sel', 'Rab', 'Kam', 'Jum', 'Sab'];

    return (
      <div className="grid grid-cols-7 gap-px bg-gray-200">
        {days.map((day) => (
          <div key={day} className="bg-gray-50 p-3 text-center">
            <span className="text-sm font-semibold text-gray-700">{day}</span>
          </div>
        ))}
      </div>
    );
  };

  const renderCells = () => {
    const monthStart = startOfMonth(currentDate);
    const monthEnd = endOfMonth(monthStart);
    const startDate = startOfWeek(monthStart, { weekStartsOn: 0 });
    const endDate = endOfWeek(monthEnd, { weekStartsOn: 0 });

    const dateFormat = "d";
    const rows = [];

    let days = [];
    let day = startDate;

    while (day <= endDate) {
      for (let i = 0; i < 7; i++) {
        const cloneDay = day;
        const dayBookings = bookings.filter(booking =>
          isSameDay(parseISO(booking.start_time), cloneDay)
        );

        const isAvailable = checkAvailability(cloneDay);

        days.push(
          <div
            key={day.toString()}
            className={`min-h-[120px] bg-white border border-gray-200 p-2 cursor-pointer hover:bg-gray-50 transition-colors ${
              !isSameMonth(day, monthStart)
                ? 'bg-gray-50 text-gray-400'
                : isToday(day)
                ? 'bg-pink-50 border-pink-300'
                : 'text-gray-900'
            }`}
            onClick={() => onDateClick(cloneDay)}
          >
            <div className="flex justify-between items-start mb-1">
              <span className={`text-sm font-medium ${
                isToday(day) ? 'text-pink-600' : ''
              }`}>
                {format(day, dateFormat)}
              </span>

              {/* Availability Indicator */}
              {isAvailable && isSameMonth(day, monthStart) && (
                <div className="flex items-center gap-1">
                  <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                  <span className="text-xs text-green-600 font-medium">Tersedia</span>
                </div>
              )}
            </div>

            {/* Bookings for this day */}
            <div className="space-y-1">
              {dayBookings.slice(0, 3).map((booking) => (
                <div
                  key={booking.id}
                  onClick={(e) => {
                    e.stopPropagation();
                    handleBookingClick(booking);
                  }}
                  className={`text-xs p-1 rounded truncate cursor-pointer hover:opacity-80 ${
                    booking.status === 'confirmed'
                      ? 'bg-green-100 text-green-800'
                      : booking.status === 'pending'
                      ? 'bg-yellow-100 text-yellow-800'
                      : booking.status === 'completed'
                      ? 'bg-blue-100 text-blue-800'
                      : 'bg-red-100 text-red-800'
                  }`}
                >
                  {format(parseISO(booking.start_time), 'HH:mm')} {booking.customer_name}
                </div>
              ))}

              {dayBookings.length > 3 && (
                <div className="text-xs text-gray-500">
                  +{dayBookings.length - 3} lainnya
                </div>
              )}
            </div>
          </div>
        );
        day = addDays(day, 1);
      }

      rows.push(
        <div className="grid grid-cols-7 gap-px" key={day.toString()}>
          {days}
        </div>
      );
      days = [];
    }

    return <div>{rows}</div>;
  };

  const checkAvailability = (date: Date): boolean => {
    const dayOfWeek = date.getDay();
    const dateString = format(date, 'yyyy-MM-dd');

    // Check for recurring availability (day of week)
    const recurringSlots = timeSlots.filter(slot =>
      slot.is_available &&
      slot.recurring &&
      slot.day_of_week === dayOfWeek
    );

    // Check for specific date availability
    const specificSlots = timeSlots.filter(slot =>
      slot.is_available &&
      !slot.recurring &&
      slot.specific_date &&
      parseISO(slot.specific_date).toDateString() === date.toDateString()
    );

    const isAvailable = recurringSlots.length > 0 || specificSlots.length > 0;

    // Debug logging for the first day of each month
    if (date.getDate() === 1) {
      console.log(`Checking availability for ${dateString}:`, {
        dayOfWeek,
        recurringSlots: recurringSlots.length,
        specificSlots: specificSlots.length,
        isAvailable,
        totalSlots: timeSlots.length
      });
    }

    return isAvailable;
  };

  const onDateClick = (day: Date) => {
    setSelectedDate(day);
    setShowAvailabilityModal(true);
  };

  const handleBookingClick = (booking: Booking) => {
    setSelectedBooking(booking);
    setShowBookingModal(true);
  };

  const handleSaveAvailability = async () => {
    setSaving(true);
    try {
      const availabilityData: CreateAvailabilityRequest = {
        start_time: availabilityForm.start_time,
        end_time: availabilityForm.end_time,
        recurring: availabilityForm.recurring,
        day_of_week: availabilityForm.recurring ? availabilityForm.day_of_week : undefined,
        specific_date: availabilityForm.recurring ? undefined :
          selectedDate ? format(selectedDate, 'yyyy-MM-dd') : undefined
      };

      const savedSlot = await dashboardApi.createAvailabilitySlot(availabilityData);

      // Update local state with the new slot
      setTimeSlots(prev => [...prev, savedSlot]);

      setShowAvailabilityModal(false);
      setAvailabilityForm({
        start_time: '09:00',
        end_time: '17:00',
        recurring: false,
        day_of_week: []
      });
    } catch (error) {
      console.error('Failed to save availability:', error);
    } finally {
      setSaving(false);
    }
  };

  const renderAvailabilityModal = () => {
    if (!showAvailabilityModal) return null;

    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
          <div className="flex justify-between items-center mb-4">
            <h3 className="text-lg font-semibold text-gray-900">
              Atur Ketersediaan
            </h3>
            <button
              onClick={() => setShowAvailabilityModal(false)}
              className="p-1 hover:bg-gray-100 rounded-lg"
            >
              <X className="w-5 h-5" />
            </button>
          </div>

          <div className="space-y-4">
            {/* Selected Date */}
            {selectedDate && (
              <div className="p-3 bg-gray-50 rounded-lg">
                <p className="text-sm text-gray-600">Tanggal</p>
                <p className="font-medium">
                  {format(selectedDate, 'EEEE, d MMMM yyyy', { locale: id })}
                </p>
              </div>
            )}

            {/* Time Range */}
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Jam Mulai
                </label>
                <input
                  type="time"
                  value={availabilityForm.start_time}
                  onChange={(e) => setAvailabilityForm(prev => ({
                    ...prev,
                    start_time: e.target.value
                  }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-pink-500 focus:border-pink-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Jam Selesai
                </label>
                <input
                  type="time"
                  value={availabilityForm.end_time}
                  onChange={(e) => setAvailabilityForm(prev => ({
                    ...prev,
                    end_time: e.target.value
                  }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-pink-500 focus:border-pink-500"
                />
              </div>
            </div>

            {/* Recurring Option */}
            <div>
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={availabilityForm.recurring}
                  onChange={(e) => setAvailabilityForm(prev => ({
                    ...prev,
                    recurring: e.target.checked,
                    day_of_week: e.target.checked ? [selectedDate?.getDay() || new Date().getDay()] : []
                  }))}
                  className="rounded border-gray-300 text-pink-600 focus:ring-pink-500"
                />
                <span className="text-sm font-medium text-gray-700">
                  Berulang setiap minggu
                </span>
              </label>
            </div>

            {/* Day Selection for Recurring */}
            {availabilityForm.recurring && (
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                  Hari
                </label>
                <div className="grid grid-cols-4 gap-2">
                  {['Min', 'Sen', 'Sel', 'Rab', 'Kam', 'Jum', 'Sab'].map((day, index) => (
                    <label key={day} className="flex items-center gap-1">
                      <input
                        type="checkbox"
                        checked={availabilityForm.day_of_week.includes(index)}
                        onChange={(e) => {
                          if (e.target.checked) {
                            setAvailabilityForm(prev => ({
                              ...prev,
                              day_of_week: [...prev.day_of_week, index]
                            }));
                          } else {
                            setAvailabilityForm(prev => ({
                              ...prev,
                              day_of_week: prev.day_of_week.filter(d => d !== index)
                            }));
                          }
                        }}
                        className="rounded border-gray-300 text-pink-600 focus:ring-pink-500"
                      />
                      <span className="text-xs">{day}</span>
                    </label>
                  ))}
                </div>
              </div>
            )}
          </div>

          <div className="flex justify-end gap-3 mt-6">
            <button
              onClick={() => setShowAvailabilityModal(false)}
              className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
            >
              Batal
            </button>
            <button
              onClick={handleSaveAvailability}
              disabled={saving}
              className="px-4 py-2 bg-pink-600 text-white rounded-lg hover:bg-pink-700 disabled:opacity-50"
            >
              {saving ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                'Simpan'
              )}
            </button>
          </div>
        </div>
      </div>
    );
  };

  const renderBookingModal = () => {
    if (!showBookingModal || !selectedBooking) return null;

    const handleUpdateBookingStatus = async (newStatus: string) => {
      try {
        await dashboardApi.updateBookingStatus(selectedBooking.id, newStatus);

        // Update local state
        setBookings(prev =>
          prev.map(booking =>
            booking.id === selectedBooking.id
              ? { ...booking, status: newStatus as any }
              : booking
          )
        );

        // Update selected booking to reflect the change
        setSelectedBooking({ ...selectedBooking, status: newStatus as any });
      } catch (error) {
        console.error('Failed to update booking status:', error);
      }
    };

    return (
      <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
        <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
          <div className="flex justify-between items-center mb-4">
            <h3 className="text-lg font-semibold text-gray-900">
              Detail Booking
            </h3>
            <button
              onClick={() => setShowBookingModal(false)}
              className="p-1 hover:bg-gray-100 rounded-lg"
            >
              <X className="w-5 h-5" />
            </button>
          </div>

          <div className="space-y-4">
            {/* Customer Info */}
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-pink-100 rounded-full flex items-center justify-center">
                <User className="w-5 h-5 text-pink-600" />
              </div>
              <div>
                <h4 className="font-medium text-gray-900">{selectedBooking.customer_name}</h4>
                <p className="text-sm text-gray-600">{selectedBooking.service_type}</p>
              </div>
            </div>

            {/* Status */}
            <div className="flex items-center gap-2">
              <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                selectedBooking.status === 'confirmed'
                  ? 'bg-green-100 text-green-800'
                  : selectedBooking.status === 'pending'
                  ? 'bg-yellow-100 text-yellow-800'
                  : selectedBooking.status === 'completed'
                  ? 'bg-blue-100 text-blue-800'
                  : 'bg-red-100 text-red-800'
              }`}>
                {selectedBooking.status.charAt(0).toUpperCase() + selectedBooking.status.slice(1)}
              </span>
            </div>

            {/* Time */}
            <div className="flex items-center gap-2">
              <Clock className="w-4 h-4 text-gray-400" />
              <div>
                <p className="text-sm font-medium">
                  {format(parseISO(selectedBooking.start_time), 'EEEE, d MMMM yyyy', { locale: id })}
                </p>
                <p className="text-sm text-gray-600">
                  {format(parseISO(selectedBooking.start_time), 'HH:mm')} - {format(parseISO(selectedBooking.end_time), 'HH:mm')}
                </p>
              </div>
            </div>

            {/* Location */}
            {selectedBooking.location && (
              <div className="flex items-center gap-2">
                <MapPin className="w-4 h-4 text-gray-400" />
                <p className="text-sm text-gray-600">{selectedBooking.location}</p>
              </div>
            )}

            {/* Phone */}
            {selectedBooking.customer_phone && (
              <div className="flex items-center gap-2">
                <Phone className="w-4 h-4 text-gray-400" />
                <p className="text-sm text-gray-600">{selectedBooking.customer_phone}</p>
              </div>
            )}

            {/* Notes */}
            {selectedBooking.notes && (
              <div>
                <p className="text-sm font-medium text-gray-700 mb-1">Catatan</p>
                <p className="text-sm text-gray-600">{selectedBooking.notes}</p>
              </div>
            )}
          </div>

          <div className="flex justify-end gap-3 mt-6">
            <button
              onClick={() => setShowBookingModal(false)}
              className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
            >
              Tutup
            </button>
            {selectedBooking.status === 'pending' && (
              <>
                <button
                  onClick={() => handleUpdateBookingStatus('cancelled')}
                  className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700"
                >
                  Tolak
                </button>
                <button
                  onClick={() => handleUpdateBookingStatus('confirmed')}
                  className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700"
                >
                  Konfirmasi
                </button>
              </>
            )}
          </div>
        </div>
      </div>
    );
  };

  if (loading) {
    return (
      <div className="flex justify-center py-12">
        <Loader2 className="w-8 h-8 animate-spin text-pink-600" />
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-sm">
      <div className="p-6">
        {renderHeader()}
        {renderDaysOfWeek()}
        {renderCells()}
      </div>

      {/* Legend */}
      <div className="px-6 pb-6 border-t">
        <div className="flex items-center gap-6 mt-4">
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-green-500 rounded-full"></div>
            <span className="text-sm text-gray-600">Tersedia</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-yellow-100 rounded"></div>
            <span className="text-sm text-gray-600">Menunggu Konfirmasi</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-green-100 rounded"></div>
            <span className="text-sm text-gray-600">Dikonfirmasi</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-blue-100 rounded"></div>
            <span className="text-sm text-gray-600">Selesai</span>
          </div>
        </div>
      </div>

      {/* Modals */}
      {renderAvailabilityModal()}
      {renderBookingModal()}
    </div>
  );
};

export default AvailabilityCalendar;