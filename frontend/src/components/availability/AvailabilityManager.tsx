import { useState, useEffect } from 'react';
import { dashboardApi } from '@/api/dashboard';
import { Clock, CheckCircle, XCircle, Loader2, Calendar as CalendarIcon } from 'lucide-react';
import AvailabilityCalendar from './AvailabilityCalendar';

const AvailabilityManager = () => {
  const [isAvailable, setIsAvailable] = useState<boolean | null>(null);
  const [loading, setLoading] = useState(true);
  const [updating, setUpdating] = useState(false);
  const [viewMode, setViewMode] = useState<'simple' | 'calendar'>('calendar');

  useEffect(() => {
    // We'll need to add an endpoint to get current availability status
    // For now, default to available
    setIsAvailable(true);
    setLoading(false);
  }, []);

  const handleToggleAvailability = async () => {
    if (isAvailable === null) return;

    try {
      setUpdating(true);
      const newStatus = !isAvailable;
      await dashboardApi.updateAvailability(newStatus);
      setIsAvailable(newStatus);
    } catch (error) {
      console.error('Failed to update availability:', error);
    } finally {
      setUpdating(false);
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center py-8">
        <Loader2 className="w-8 h-8 animate-spin text-pink-600" />
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header with View Toggle */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold text-gray-900">Kelola Ketersediaan</h2>
          <p className="text-sm text-gray-600 mt-1">
            Atur jadwal dan kelola booking Anda
          </p>
        </div>

        <div className="flex items-center gap-4">
          {/* View Mode Toggle */}
          <div className="flex items-center bg-gray-100 rounded-lg p-1">
            <button
              onClick={() => setViewMode('simple')}
              className={`px-3 py-1 rounded-md text-sm font-medium transition-colors ${
                viewMode === 'simple'
                  ? 'bg-white text-pink-600 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              Sederhana
            </button>
            <button
              onClick={() => setViewMode('calendar')}
              className={`px-3 py-1 rounded-md text-sm font-medium transition-colors ${
                viewMode === 'calendar'
                  ? 'bg-white text-pink-600 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <CalendarIcon className="w-4 h-4 inline mr-1" />
              Kalender
            </button>
          </div>

          {/* Quick Availability Toggle */}
          <button
            onClick={handleToggleAvailability}
            disabled={updating}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
              isAvailable ? 'bg-green-600' : 'bg-gray-200'
            } ${updating ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
          >
            <span
              className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                isAvailable ? 'translate-x-6' : 'translate-x-1'
              }`}
            />
            {updating && (
              <Loader2 className="absolute inset-0 m-auto w-3 h-3 animate-spin text-white" />
            )}
          </button>
        </div>
      </div>

      {/* Calendar View */}
      {viewMode === 'calendar' && (
        <AvailabilityCalendar />
      )}

      {/* Simple View */}
      {viewMode === 'simple' && (
        <div className="bg-white rounded-lg shadow-sm p-6">
          <div className={`rounded-lg p-6 border-2 ${
            isAvailable
              ? 'bg-green-50 border-green-200'
              : 'bg-red-50 border-red-200'
          }`}>
            <div className="flex items-center gap-3">
              {isAvailable ? (
                <CheckCircle className="w-8 h-8 text-green-600" />
              ) : (
                <XCircle className="w-8 h-8 text-red-600" />
              )}
              <div>
                <h3 className={`text-lg font-semibold ${
                  isAvailable ? 'text-green-900' : 'text-red-900'
                }`}>
                  {isAvailable ? 'Sedang Tersedia' : 'Sedang Tidak Tersedia'}
                </h3>
                <p className={`text-sm mt-1 ${
                  isAvailable ? 'text-green-700' : 'text-red-700'
                }`}>
                  {isAvailable
                    ? 'Anda dapat menerima booking baru dari pelanggan'
                    : 'Anda tidak akan menerima booking baru sampai status diubah'
                  }
                </p>
              </div>
            </div>
          </div>

          <div className="mt-6 space-y-4">
            <h3 className="font-medium text-gray-900 flex items-center gap-2">
              <Clock className="w-4 h-4" />
              Tips Ketersediaan
            </h3>

            <div className="space-y-3">
              <div className="flex items-start gap-3">
                <div className="w-2 h-2 bg-pink-600 rounded-full mt-1.5"></div>
                <div>
                  <p className="text-sm font-medium text-gray-900">
                    Gunakan Kalender View
                  </p>
                  <p className="text-xs text-gray-600">
                    Atur jadwal mingguan dan lihat booking dalam bentuk kalender
                  </p>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <div className="w-2 h-2 bg-pink-600 rounded-full mt-1.5"></div>
                <div>
                  <p className="text-sm font-medium text-gray-900">
                    Atur Jadwal Reguler
                  </p>
                  <p className="text-xs text-gray-600">
                    Usahakan untuk konsisten dengan jam operasional Anda
                  </p>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <div className="w-2 h-2 bg-pink-600 rounded-full mt-1.5"></div>
                <div>
                  <p className="text-sm font-medium text-gray-900">
                    Update Status Secara Berkala
                  </p>
                  <p className="text-xs text-gray-600">
                    Segera ubah status jika ada perubahan jadwal mendadak
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="mt-6 pt-6 border-t">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-900">Status Saat Ini</p>
                <p className="text-xs text-gray-500 mt-1">
                  Terakhir diupdate: {new Date().toLocaleString('id-ID')}
                </p>
              </div>
              <span className={`inline-flex items-center gap-1 px-3 py-1 rounded-full text-xs font-medium ${
                isAvailable
                  ? 'bg-green-100 text-green-800'
                  : 'bg-red-100 text-red-800'
              }`}>
                {isAvailable ? (
                  <>
                    <CheckCircle className="w-3 h-3" />
                    Tersedia
                  </>
                ) : (
                  <>
                    <XCircle className="w-3 h-3" />
                    Tidak Tersedia
                  </>
                )}
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AvailabilityManager;