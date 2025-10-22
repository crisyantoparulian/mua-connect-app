import { useState } from 'react';
import { dashboardApi, type BookingCalendar } from '@/api/dashboard';
import { X, Calendar, Clock, MapPin, User, Phone, DollarSign, Check, X as XIcon, Loader2 } from 'lucide-react';

interface BookingDetailModalProps {
  booking: BookingCalendar | null;
  isOpen: boolean;
  onClose: () => void;
  onStatusUpdate: () => void;
}

const BookingDetailModal: React.FC<BookingDetailModalProps> = ({
  booking,
  isOpen,
  onClose,
  onStatusUpdate,
}) => {
  const [loading, setLoading] = useState(false);
  const [rejectionReason, setRejectionReason] = useState('');

  if (!isOpen || !booking) {
    return null;
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
      month: 'long',
      year: 'numeric',
    });
  };

  const formatTime = (dateString: string) => {
    return new Date(dateString).toLocaleTimeString('id-ID', {
      hour: '2-digit',
      minute: '2-digit',
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

  const handleAccept = async () => {
    try {
      setLoading(true);
      await dashboardApi.updateBookingStatus(booking.id, 'confirmed');
      onStatusUpdate();
      onClose();
    } catch (error) {
      console.error('Failed to accept booking:', error);
      alert('Gagal menerima booking. Silakan coba lagi.');
    } finally {
      setLoading(false);
    }
  };

  const handleReject = async () => {
    if (!rejectionReason.trim()) {
      alert('Silakan masukkan alasan penolakan.');
      return;
    }

    try {
      setLoading(true);
      await dashboardApi.updateBookingStatus(booking.id, 'cancelled');
      onStatusUpdate();
      onClose();
      setRejectionReason('');
    } catch (error) {
      console.error('Failed to reject booking:', error);
      alert('Gagal menolak booking. Silakan coba lagi.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
      <div className="bg-white rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto">
        <div className="sticky top-0 bg-white border-b p-6">
          <div className="flex items-center justify-between">
            <h3 className="text-xl font-semibold text-gray-900">Detail Booking</h3>
            <button
              onClick={onClose}
              className="text-gray-400 hover:text-gray-600"
            >
              <X className="w-6 h-6" />
            </button>
          </div>
        </div>

        <div className="p-6">
          <div className="flex items-center justify-between mb-6">
            <span className={`px-3 py-1 rounded-full text-sm font-medium ${getStatusColor(booking.status)}`}>
              {booking.status.charAt(0).toUpperCase() + booking.status.slice(1)}
            </span>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h4 className="font-medium text-gray-900 mb-4">Informasi Pelanggan</h4>
              <div className="space-y-3">
                <div className="flex items-center text-gray-600">
                  <User className="w-4 h-4 mr-3 text-gray-400" />
                  <span>{booking.customer_name}</span>
                </div>
                {booking.customer_phone && (
                  <div className="flex items-center text-gray-600">
                    <Phone className="w-4 h-4 mr-3 text-gray-400" />
                    <span>{booking.customer_phone}</span>
                  </div>
                )}
              </div>
            </div>

            <div>
              <h4 className="font-medium text-gray-900 mb-4">Informasi Acara</h4>
              <div className="space-y-3">
                <div className="flex items-center text-gray-600">
                  <Calendar className="w-4 h-4 mr-3 text-gray-400" />
                  <span>{formatDate(booking.start_time)}</span>
                </div>
                <div className="flex items-center text-gray-600">
                  <Clock className="w-4 h-4 mr-3 text-gray-400" />
                  <span>{formatTime(booking.start_time)} - {formatTime(booking.end_time)}</span>
                </div>
                {booking.location && (
                  <div className="flex items-center text-gray-600">
                    <MapPin className="w-4 h-4 mr-3 text-gray-400" />
                    <span>{booking.location}</span>
                  </div>
                )}
              </div>
            </div>
          </div>

          <div className="mt-6">
            <h4 className="font-medium text-gray-900 mb-2">Detail Layanan</h4>
            <p className="text-gray-600">{booking.service_type}</p>
          </div>

          {booking.notes && (
            <div className="mt-6">
              <h4 className="font-medium text-gray-900 mb-2">Catatan Tambahan</h4>
              <p className="text-gray-600 whitespace-pre-wrap">{booking.notes}</p>
            </div>
          )}

          {booking.status === 'pending' && (
            <div className="mt-8 border-t pt-6">
              <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mb-6">
                <h4 className="font-medium text-yellow-800 mb-2">Booking Menunggu Konfirmasi</h4>
                <p className="text-sm text-yellow-700">
                  Pelanggan menunggu konfirmasi Anda untuk booking ini. Segera terima atau tolak booking ini.
                </p>
              </div>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">
                    Alasan Penolakan (wajib diisi jika menolak)
                  </label>
                  <textarea
                    value={rejectionReason}
                    onChange={(e) => setRejectionReason(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                    rows={3}
                    placeholder="Contoh: Jadwal sudah penuh, lokasi terlalu jauh, dll..."
                  />
                  {rejectionReason && (
                    <p className="text-xs text-gray-500 mt-1">
                      {rejectionReason.length}/100 karakter
                    </p>
                  )}
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <button
                    onClick={handleAccept}
                    disabled={loading}
                    className="bg-green-600 text-white py-3 px-6 rounded-md hover:bg-green-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center font-medium shadow-lg hover:shadow-xl transform hover:scale-105 transition-all"
                  >
                    {loading ? (
                      <Loader2 className="w-5 h-5 animate-spin mr-2" />
                    ) : (
                      <Check className="w-5 h-5 mr-2" />
                    )}
                    ✅ Terima Booking
                  </button>
                  <button
                    onClick={handleReject}
                    disabled={loading || !rejectionReason.trim()}
                    className="bg-red-600 text-white py-3 px-6 rounded-md hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center font-medium shadow-lg hover:shadow-xl transform hover:scale-105 transition-all"
                  >
                    {loading ? (
                      <Loader2 className="w-5 h-5 animate-spin mr-2" />
                    ) : (
                      <XIcon className="w-5 h-5 mr-2" />
                    )}
                    ❌ Tolak Booking
                  </button>
                </div>

                <div className="text-xs text-gray-500 text-center">
                  <p>• Terima booking untuk mengonfirmasi jadwal dengan pelanggan</p>
                  <p>• Tolak booking jika Anda tidak dapat melayani pada jadwal tersebut</p>
                </div>
              </div>
            </div>
          )}

          {booking.status === 'confirmed' && (
            <div className="mt-8 border-t pt-6">
              <div className="bg-green-50 border border-green-200 rounded-lg p-4">
                <h4 className="font-medium text-green-800 mb-2">Booking Telah Dikonfirmasi</h4>
                <p className="text-sm text-green-700">
                  Booking ini telah dikonfirmasi. Pastikan untuk siap pada waktu yang telah ditentukan.
                </p>
              </div>
            </div>
          )}

          {booking.status === 'cancelled' && (
            <div className="mt-8 border-t pt-6">
              <div className="bg-red-50 border border-red-200 rounded-lg p-4">
                <h4 className="font-medium text-red-800 mb-2">Booking Ditolak</h4>
                <p className="text-sm text-red-700">
                  Booking ini telah ditolak. Pelanggan telah diberitahu tentang penolakan ini.
                </p>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default BookingDetailModal;