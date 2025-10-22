import { useAuthStore } from '@/store/authStore';
import { Navigate, useSearchParams } from 'react-router-dom';
import { Calendar, Users, Star, DollarSign, Loader2, LayoutDashboard, Image, Clock, CalendarDays } from 'lucide-react';
import { useState, useEffect } from 'react';
import { dashboardApi, type BookingCalendar } from '@/api/dashboard';
import PortfolioManager from '@/components/portfolio/PortfolioManager';
import AvailabilityManager from '@/components/availability/AvailabilityManager';
import CalendarView from '@/components/calendar/CalendarView';
import BookingDetailModal from '@/components/calendar/BookingDetailModal';

interface DashboardStats {
  total_bookings: number;
  pending_bookings: number;
  confirmed_bookings: number;
  completed_bookings: number;
  total_revenue: number;
  average_rating?: number;
  total_reviews: number;
  portfolio_items: number;
}

interface RecentBooking {
  id: string;
  customer_name: string;
  service_type: string;
  event_date: string;
  status: 'pending' | 'confirmed' | 'completed' | 'cancelled';
  price: number;
}

interface DashboardResponse {
  stats: DashboardStats;
  recent_bookings: RecentBooking[];
  upcoming_bookings: RecentBooking[];
}

const Dashboard = () => {
  const { user } = useAuthStore();
  const [searchParams, setSearchParams] = useSearchParams();
  const [dashboardData, setDashboardData] = useState<DashboardResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Get initial tab from URL params, default to 'overview'
  const getInitialTab = (): 'overview' | 'portfolio' | 'availability' | 'calendar' => {
    const tab = searchParams.get('tab');
    return (tab === 'portfolio' || tab === 'availability' || tab === 'calendar') ? tab : 'overview';
  };

  const [activeTab, setActiveTab] = useState<'overview' | 'portfolio' | 'availability' | 'calendar'>(getInitialTab);
  const [selectedBooking, setSelectedBooking] = useState<BookingCalendar | null>(null);
  const [showBookingModal, setShowBookingModal] = useState(false);

  // Update URL when tab changes
  useEffect(() => {
    const currentTab = searchParams.get('tab');
    if (currentTab !== activeTab) {
      if (activeTab === 'overview') {
        searchParams.delete('tab');
      } else {
        searchParams.set('tab', activeTab);
      }
      setSearchParams(searchParams);
    }
  }, [activeTab, searchParams, setSearchParams]);

  useEffect(() => {
    const fetchDashboardData = async () => {
      try {
        setLoading(true);
        const data = await dashboardApi.getDashboard();
        setDashboardData(data);
      } catch (err: any) {
        console.error('Failed to fetch dashboard data:', err);
        if (err.response?.status === 404) {
          setError('MUA profile not found. Please complete your MUA registration first.');
        } else if (err.response?.status === 401) {
          setError('Please login to access the dashboard.');
        } else {
          setError('Failed to load dashboard data. Please try again.');
        }
      } finally {
        setLoading(false);
      }
    };

    fetchDashboardData();
  }, []);

  if (!user || user.user_type !== 'mua') {
    return <Navigate to="/" replace />;
  }

  if (loading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <Loader2 className="w-8 h-8 animate-spin text-pink-600" />
      </div>
    );
  }

  if (error || !dashboardData) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <p className="text-red-600 mb-4">{error || 'No data available'}</p>
          <button
            onClick={() => window.location.reload()}
            className="px-4 py-2 bg-pink-600 text-white rounded-lg hover:bg-pink-700"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  const formatCurrency = (amount: number) => {
    return new Intl.NumberFormat('id-ID', {
      style: 'currency',
      currency: 'IDR',
      minimumFractionDigits: 0,
    }).format(amount);
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('id-ID', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
    });
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pending':
        return 'bg-yellow-100 text-yellow-800';
      case 'confirmed':
        return 'bg-green-100 text-green-800';
      case 'completed':
        return 'bg-blue-100 text-blue-800';
      case 'cancelled':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const handleBookingClick = (booking: BookingCalendar) => {
    setSelectedBooking(booking);
    setShowBookingModal(true);
  };

  const handleBookingStatusUpdate = () => {
    // Refresh dashboard data when booking status is updated
    const fetchDashboardData = async () => {
      try {
        const data = await dashboardApi.getDashboard();
        setDashboardData(data);
      } catch (err) {
        console.error('Failed to refresh dashboard data:', err);
      }
    };
    fetchDashboardData();
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900">Dashboard MUA</h1>
          <p className="text-gray-600 mt-2">
            Kelola bisnis makeup artis Anda
          </p>
        </div>

        {/* Tab Navigation */}
        <div className="mb-8 border-b border-gray-200">
          <nav className="-mb-px flex space-x-8">
            <button
              onClick={() => setActiveTab('overview')}
              className={`flex items-center gap-2 py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'overview'
                  ? 'border-pink-500 text-pink-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              <LayoutDashboard className="w-4 h-4" />
              Ringkasan
            </button>
            <button
              onClick={() => setActiveTab('portfolio')}
              className={`flex items-center gap-2 py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'portfolio'
                  ? 'border-pink-500 text-pink-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              <Image className="w-4 h-4" />
              Portofolio
            </button>
            <button
              onClick={() => setActiveTab('availability')}
              className={`flex items-center gap-2 py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'availability'
                  ? 'border-pink-500 text-pink-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              <Clock className="w-4 h-4" />
              Ketersediaan
            </button>
            <button
              onClick={() => setActiveTab('calendar')}
              className={`flex items-center gap-2 py-2 px-1 border-b-2 font-medium text-sm ${
                activeTab === 'calendar'
                  ? 'border-pink-500 text-pink-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              }`}
            >
              <CalendarDays className="w-4 h-4" />
              Kalender
            </button>
          </nav>
        </div>

        {/* Tab Content */}
        {activeTab === 'overview' && (
          <div>

        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-lg shadow-sm p-6">
            <div className="flex items-center">
              <div className="p-2 bg-pink-100 rounded-lg">
                <Calendar className="w-6 h-6 text-pink-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Total Booking</p>
                <p className="text-2xl font-bold text-gray-900">{dashboardData.stats.total_bookings}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow-sm p-6">
            <div className="flex items-center">
              <div className="p-2 bg-green-100 rounded-lg">
                <DollarSign className="w-6 h-6 text-green-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Pendapatan</p>
                <p className="text-2xl font-bold text-gray-900">{formatCurrency(dashboardData.stats.total_revenue)}</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow-sm p-6">
            <div className="flex items-center">
              <div className="p-2 bg-yellow-100 rounded-lg">
                <Star className="w-6 h-6 text-yellow-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Rating</p>
                <p className="text-2xl font-bold text-gray-900">
                  {dashboardData.stats.average_rating?.toFixed(1) || 'N/A'}
                </p>
                <p className="text-xs text-gray-500">({dashboardData.stats.total_reviews} reviews)</p>
              </div>
            </div>
          </div>

          <div className="bg-white rounded-lg shadow-sm p-6">
            <div className="flex items-center">
              <div className="p-2 bg-blue-100 rounded-lg">
                <Users className="w-6 h-6 text-blue-600" />
              </div>
              <div className="ml-4">
                <p className="text-sm font-medium text-gray-600">Portofolio</p>
                <p className="text-2xl font-bold text-gray-900">{dashboardData.stats.portfolio_items}</p>
              </div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Recent Bookings */}
          <div className="bg-white rounded-lg shadow-sm p-6">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">
              Booking Terbaru
            </h2>
            <div className="space-y-4">
              {dashboardData.recent_bookings.length > 0 ? (
                dashboardData.recent_bookings.map((booking: RecentBooking) => (
                  <div key={booking.id} className="border-b pb-4 last:border-b-0">
                    <div className="flex justify-between items-start">
                      <div>
                        <h3 className="font-medium text-gray-900">{booking.customer_name}</h3>
                        <p className="text-sm text-gray-600">{booking.service_type}</p>
                        <p className="text-sm text-gray-500">{formatDate(booking.event_date)}</p>
                      </div>
                      <span className={`px-2 py-1 text-xs font-medium rounded-full ${getStatusColor(booking.status)}`}>
                        {booking.status.charAt(0).toUpperCase() + booking.status.slice(1)}
                      </span>
                    </div>
                  </div>
                ))
              ) : (
                <p className="text-gray-500 text-center py-4">Belum ada booking</p>
              )}
            </div>
          </div>

          {/* Quick Actions */}
          <div className="bg-white rounded-lg shadow-sm p-6">
            <h2 className="text-xl font-semibold text-gray-900 mb-4">
              Aksi Cepat
            </h2>
            <div className="space-y-3">
              <button
                onClick={() => setActiveTab('availability')}
                className="w-full text-left px-4 py-3 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors"
              >
                <h3 className="font-medium text-gray-900">Kelola Ketersediaan</h3>
                <p className="text-sm text-gray-600">Atur jadwal kerja Anda</p>
              </button>

              <button
                onClick={() => setActiveTab('portfolio')}
                className="w-full text-left px-4 py-3 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors"
              >
                <h3 className="font-medium text-gray-900">Tambah Portofolio</h3>
                <p className="text-sm text-gray-600">Unggah karya terbaru</p>
              </button>

              <button
                onClick={() => setActiveTab('calendar')}
                className="w-full text-left px-4 py-3 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors"
              >
                <h3 className="font-medium text-gray-900">Lihat Kalender</h3>
                <p className="text-sm text-gray-600">Kelola semua booking di kalender</p>
              </button>

              <button className="w-full text-left px-4 py-3 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors">
                <h3 className="font-medium text-gray-900">Lihat Review</h3>
                <p className="text-sm text-gray-600">Baca ulasan dari pelanggan</p>
              </button>

              <button className="w-full text-left px-4 py-3 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors">
                <h3 className="font-medium text-gray-900">Pengaturan Layanan</h3>
                <p className="text-sm text-gray-600">Update harga dan layanan</p>
              </button>
            </div>
          </div>
        </div>
          </div>
        )}

        {activeTab === 'portfolio' && (
          <PortfolioManager />
        )}

        {activeTab === 'availability' && (
          <AvailabilityManager />
        )}

        {activeTab === 'calendar' && (
          <CalendarView onBookingClick={handleBookingClick} />
        )}
      </div>

      {/* Booking Detail Modal */}
      <BookingDetailModal
        booking={selectedBooking}
        isOpen={showBookingModal}
        onClose={() => {
          setShowBookingModal(false);
          setSelectedBooking(null);
        }}
        onStatusUpdate={handleBookingStatusUpdate}
      />
    </div>
  );
};

export default Dashboard;