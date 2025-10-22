import { useParams, useNavigate } from 'react-router-dom';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { muasApi } from '@/api/muas';
import { bookingsApi, type CreateBookingRequest } from '@/api/bookings';
import type { MuaProfile } from '@/types';
import { Star, MapPin, Calendar, Clock, ArrowLeft, User, Mail, Phone, Camera, Heart, X, MessageCircle, Send, Loader2 } from 'lucide-react';

const MuaProfile = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const [selectedPortfolio, setSelectedPortfolio] = useState<any>(null);
  const [showContactModal, setShowContactModal] = useState(false);
  const [showBookingModal, setShowBookingModal] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [contactMessage, setContactMessage] = useState('');
  const [bookingData, setBookingData] = useState({
    event_date: '',
    event_location: '',
    duration_hours: 1,
    description: '',
    service_type: '',
  });

  // Booking mutation
  const createBookingMutation = useMutation({
    mutationFn: (bookingData: CreateBookingRequest) => bookingsApi.createBooking(bookingData),
    onSuccess: () => {
      setShowBookingModal(false);
      setShowSuccessModal(true);
      setBookingData({
        event_date: '',
        event_location: '',
        duration_hours: 1,
        description: '',
        service_type: '',
      });
      // Invalidate bookings query to refresh calendar if MUA views it
      queryClient.invalidateQueries({ queryKey: ['calendarBookings'] });
    },
    onError: (error: any) => {
      console.error('Failed to create booking:', error);
      alert('Gagal membuat pesanan. Silakan coba lagi.');
    },
  });

  const handleBookingSubmit = () => {
    // Validate form
    if (!bookingData.event_date || !bookingData.event_date.includes('T')) {
      alert('Silakan pilih tanggal dan waktu acara');
      return;
    }
    if (!bookingData.event_location) {
      alert('Silakan masukkan lokasi acara');
      return;
    }
    if (!bookingData.service_type) {
      alert('Silakan pilih jenis layanan');
      return;
    }

    // Calculate price based on duration and service type (you can adjust this logic)
    const basePrice = {
      'Bridal Makeup': 2500000,
      'Party Makeup': 800000,
      'Photoshoot Makeup': 1200000,
      'Lainnya': 1000000
    };

    const price = (basePrice[bookingData.service_type as keyof typeof basePrice] || 1000000) * bookingData.duration_hours;

    const bookingRequest: CreateBookingRequest = {
      mua_id: id!,
      service_type: bookingData.service_type,
      description: bookingData.description,
      event_date: new Date(bookingData.event_date).toISOString(),
      event_location: bookingData.event_location,
      duration_hours: bookingData.duration_hours,
      price: price.toString(),
    };

    createBookingMutation.mutate(bookingRequest);
  };

  const { data: mua, isLoading, error } = useQuery({
    queryKey: ['mua', id],
    queryFn: () => muasApi.getMuaById(id!),
    enabled: !!id,
  });

  const { data: portfolio, isLoading: isPortfolioLoading } = useQuery({
    queryKey: ['portfolio', id],
    queryFn: () => muasApi.getMuaPortfolio(id!),
    enabled: !!id,
  });

  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-pink-600"></div>
          <p className="mt-2 text-gray-600">Memuat profil MUA...</p>
        </div>
      </div>
    );
  }

  if (error || !mua) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <h2 className="text-2xl font-bold text-gray-900 mb-4">MUA Tidak Ditemukan</h2>
          <p className="text-gray-600 mb-6">Maaf, MUA yang Anda cari tidak tersedia.</p>
          <button
            onClick={() => navigate('/search')}
            className="bg-pink-600 text-white px-6 py-2 rounded-md hover:bg-pink-700 transition-colors"
          >
            Kembali ke Pencarian
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Back Button */}
        <button
          onClick={() => navigate('/search')}
          className="flex items-center text-gray-600 hover:text-gray-900 mb-6"
        >
          <ArrowLeft className="w-5 h-5 mr-2" />
          Kembali ke Pencarian
        </button>

        {/* Profile Header */}
        <div className="bg-white rounded-lg shadow-sm overflow-hidden">
          <div className="p-6">
            <div className="flex items-start space-x-6">
              <div className="w-24 h-24 bg-pink-100 rounded-full flex items-center justify-center">
                <User className="w-12 h-12 text-pink-600" />
              </div>

              <div className="flex-1">
                <h1 className="text-3xl font-bold text-gray-900 mb-2">
                  {mua.user.full_name}
                </h1>

                <div className="flex items-center text-gray-600 mb-4">
                  <Star className="w-5 h-5 text-yellow-400 fill-current mr-1" />
                  <span className="font-medium">{mua.average_rating || '0.0'}</span>
                  <span className="mx-2">•</span>
                  <span>{mua.total_reviews || 0} review</span>
                  <span className="mx-2">•</span>
                  <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                    mua.is_available
                      ? 'bg-green-100 text-green-800'
                      : 'bg-red-100 text-red-800'
                  }`}>
                    {mua.is_available ? 'Tersedia' : 'Tidak Tersedia'}
                  </span>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-gray-600">
                  <div className="flex items-center">
                    <Mail className="w-4 h-4 mr-2" />
                    <span>{mua.user.email}</span>
                  </div>
                  {mua.user.phone_number && (
                    <div className="flex items-center">
                      <Phone className="w-4 h-4 mr-2" />
                      <span>{mua.user.phone_number}</span>
                    </div>
                  )}
                  <div className="flex items-center">
                    <MapPin className="w-4 h-4 mr-2" />
                    <span>{mua.location || 'Lokasi tidak tersedia'}</span>
                  </div>
                  {mua.experience_years && (
                    <div className="flex items-center">
                      <Clock className="w-4 h-4 mr-2" />
                      <span>{mua.experience_years} tahun pengalaman</span>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Bio and Specialization */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-6">
          <div className="lg:col-span-2">
            <div className="bg-white rounded-lg shadow-sm p-6">
              <h2 className="text-xl font-semibold text-gray-900 mb-4">Tentang</h2>
              {mua.bio ? (
                <p className="text-gray-600 whitespace-pre-wrap">{mua.bio}</p>
              ) : (
                <p className="text-gray-400 italic">Belum ada bio</p>
              )}
            </div>

            {/* Specialization */}
            {mua.specialization && mua.specialization.length > 0 && (
              <div className="bg-white rounded-lg shadow-sm p-6 mt-6">
                <h2 className="text-xl font-semibold text-gray-900 mb-4">Spesialisasi</h2>
                <div className="flex flex-wrap gap-2">
                  {mua.specialization.map((spec, index) => (
                    <span
                      key={index}
                      className="px-3 py-2 bg-pink-100 text-pink-700 rounded-full text-sm font-medium"
                    >
                      {spec}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {/* Portfolio */}
            <div className="bg-white rounded-lg shadow-sm p-6 mt-6">
              <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold text-gray-900">Portfolio</h2>
                <div className="flex items-center text-sm text-gray-600">
                  <Camera className="w-4 h-4 mr-1" />
                  <span>{portfolio?.length || 0} karya</span>
                </div>
              </div>

              {isPortfolioLoading ? (
                <div className="text-center py-8">
                  <div className="inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-pink-600"></div>
                  <p className="mt-2 text-gray-600">Memuat portfolio...</p>
                </div>
              ) : portfolio && portfolio.length > 0 ? (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {portfolio.map((item) => (
                    <div key={item.id} className="group relative overflow-hidden rounded-lg border border-gray-200 hover:shadow-md transition-shadow">
                      <div className="aspect-w-16 aspect-h-12 bg-gray-100 relative">
                        <img
                          src={item.image_url}
                          alt={item.title}
                          className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                          onError={(e) => {
                            const target = e.target as HTMLImageElement;
                            target.src = 'https://picsum.photos/seed/portfolio-mua/400/300.jpg';
                          }}
                        />
                        <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
                      </div>
                      <div className="p-4">
                        <h3 className="font-medium text-gray-900 mb-1">{item.title}</h3>
                        {item.service_type && (
                          <span className="inline-block px-2 py-1 bg-pink-100 text-pink-700 rounded-full text-xs mb-2">
                            {item.service_type}
                          </span>
                        )}
                        {item.description && (
                          <p className="text-sm text-gray-600 line-clamp-2">{item.description}</p>
                        )}
                        <div className="flex items-center justify-between mt-3">
                          <span className="text-xs text-gray-500">
                            {new Date(item.created_at).toLocaleDateString('id-ID')}
                          </span>
                          <div className="flex gap-2">
                            <button
                              onClick={() => setSelectedPortfolio(item)}
                              className="text-pink-600 hover:text-pink-700 hover:scale-110 transition-transform"
                            >
                              <Camera className="w-4 h-4" />
                            </button>
                            <button className="text-pink-600 hover:text-pink-700">
                              <Heart className="w-4 h-4" />
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8">
                  <div className="text-gray-400 mb-4">
                    <Camera className="w-16 h-16 mx-auto" />
                  </div>
                  <h3 className="text-lg font-medium text-gray-900 mb-2">Belum Ada Portfolio</h3>
                  <p className="text-gray-600">MUA ini belum menambahkan portfolio karyanya</p>
                </div>
              )}
            </div>
          </div>

          {/* Contact Section */}
          <div className="lg:col-span-1">
            <div className="bg-white rounded-lg shadow-sm p-6">
              <h2 className="text-xl font-semibold text-gray-900 mb-4">Kontak</h2>
              <div className="space-y-3">
                <button
                  onClick={() => setShowBookingModal(true)}
                  className="w-full bg-pink-600 text-white py-3 px-4 rounded-md hover:bg-pink-700 transition-colors font-medium"
                >
                  Pesan Sekarang
                </button>
                <button
                  onClick={() => setShowContactModal(true)}
                  className="w-full border border-pink-600 text-pink-600 py-3 px-4 rounded-md hover:bg-pink-50 transition-colors font-medium"
                >
                  Hubungi Langsung
                </button>
              </div>

              <div className="mt-6 text-sm text-gray-500">
                <p>Bergabung sejak {new Date(mua.created_at).toLocaleDateString('id-ID')}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Portfolio Detail Modal */}
      {selectedPortfolio && (
        <div className="fixed inset-0 bg-black bg-opacity-75 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-4xl max-h-[90vh] overflow-hidden">
            <div className="relative">
              <button
                onClick={() => setSelectedPortfolio(null)}
                className="absolute top-4 right-4 z-10 bg-white rounded-full p-2 shadow-lg hover:bg-gray-100"
              >
                <X className="w-6 h-6 text-gray-600" />
              </button>

              <div className="grid grid-cols-1 lg:grid-cols-2">
                <div className="aspect-square bg-gray-100">
                  <img
                    src={selectedPortfolio.image_url}
                    alt={selectedPortfolio.title}
                    className="w-full h-full object-cover"
                    onError={(e) => {
                      const target = e.target as HTMLImageElement;
                      target.src = 'https://picsum.photos/seed/portfolio-profile/600/600.jpg';
                    }}
                  />
                </div>

                <div className="p-6">
                  <h3 className="text-2xl font-bold text-gray-900 mb-4">
                    {selectedPortfolio.title}
                  </h3>

                  {selectedPortfolio.service_type && (
                    <span className="inline-block px-3 py-2 bg-pink-100 text-pink-700 rounded-full text-sm font-medium mb-4">
                      {selectedPortfolio.service_type}
                    </span>
                  )}

                  {selectedPortfolio.description && (
                    <div className="mb-6">
                      <h4 className="text-lg font-semibold text-gray-900 mb-2">Deskripsi</h4>
                      <p className="text-gray-600 whitespace-pre-wrap">
                        {selectedPortfolio.description}
                      </p>
                    </div>
                  )}

                  <div className="text-sm text-gray-500">
                    <p>Diunggah pada {new Date(selectedPortfolio.created_at).toLocaleDateString('id-ID', {
                      year: 'numeric',
                      month: 'long',
                      day: 'numeric'
                    })}</p>
                  </div>

                  <div className="mt-6 flex gap-3">
                    <button className="flex-1 bg-pink-600 text-white py-3 px-4 rounded-md hover:bg-pink-700 transition-colors font-medium">
                      <Heart className="w-5 h-5 inline mr-2" />
                      Suka
                    </button>
                    <button
                      onClick={() => {
                        setSelectedPortfolio(null);
                        setShowBookingModal(true);
                      }}
                      className="flex-1 border border-pink-600 text-pink-600 py-3 px-4 rounded-md hover:bg-pink-50 transition-colors font-medium"
                    >
                      <Send className="w-5 h-5 inline mr-2" />
                      Pesan Layanan Ini
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Contact Modal */}
      {showContactModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-md w-full p-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-xl font-semibold text-gray-900">Hubungi {mua.user.full_name}</h3>
              <button
                onClick={() => setShowContactModal(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                <X className="w-6 h-6" />
              </button>
            </div>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Pesan Anda
                </label>
                <textarea
                  value={contactMessage}
                  onChange={(e) => setContactMessage(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                  rows={4}
                  placeholder="Halo, saya tertarik dengan layanan Anda..."
                />
              </div>

              <div className="flex gap-3">
                <button
                  onClick={() => setShowContactModal(false)}
                  className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors"
                >
                  Batal
                </button>
                <button
                  onClick={() => {
                    console.log('Sending message:', contactMessage);
                    setShowContactModal(false);
                    setContactMessage('');
                  }}
                  className="flex-1 bg-pink-600 text-white px-4 py-2 rounded-md hover:bg-pink-700 transition-colors"
                >
                  <Send className="w-4 h-4 inline mr-2" />
                  Kirim Pesan
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Booking Modal */}
      {showBookingModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-md w-full p-6 max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-xl font-semibold text-gray-900">Pesan Layanan {mua.user.full_name}</h3>
              <button
                onClick={() => setShowBookingModal(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                <X className="w-6 h-6" />
              </button>
            </div>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Tanggal Acara
                </label>
                <input
                  type="date"
                  value={bookingData.event_date ? bookingData.event_date.split('T')[0] : ''}
                  min={new Date().toISOString().split('T')[0]} // Prevent past dates
                  onChange={(e) => {
                    const currentDate = bookingData.event_date || '';
                    const currentTime = currentDate.includes('T') ? currentDate.split('T')[1] : '09:00';
                    const newDateTime = `${e.target.value}T${currentTime}`;
                    setBookingData(prev => ({ ...prev, event_date: newDateTime }));
                  }}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Waktu Acara
                </label>
                <input
                  type="time"
                  value={bookingData.event_date && bookingData.event_date.includes('T') ? bookingData.event_date.split('T')[1].substring(0, 5) : '09:00'}
                  onChange={(e) => {
                    const currentDate = bookingData.event_date || new Date().toISOString().split('T')[0];
                    const newDateTime = `${currentDate.split('T')[0]}T${e.target.value}:00`;
                    setBookingData(prev => ({ ...prev, event_date: newDateTime }));
                  }}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Lokasi Acara
                </label>
                <input
                  type="text"
                  value={bookingData.event_location}
                  onChange={(e) => setBookingData(prev => ({ ...prev, event_location: e.target.value }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                  placeholder="Alamat lengkap acara"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Durasi (Jam)
                </label>
                <select
                  value={bookingData.duration_hours}
                  onChange={(e) => setBookingData(prev => ({ ...prev, duration_hours: Number(e.target.value) }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                >
                  <option value={1}>1 Jam</option>
                  <option value={2}>2 Jam</option>
                  <option value={3}>3 Jam</option>
                  <option value={4}>4 Jam</option>
                  <option value={5}>5 Jam</option>
                  <option value={6}>6 Jam</option>
                  <option value={7}>7 Jam</option>
                  <option value={8}>8 Jam</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Jenis Layanan
                </label>
                <select
                  value={bookingData.service_type}
                  onChange={(e) => setBookingData(prev => ({ ...prev, service_type: e.target.value }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                >
                  <option value="">Pilih Jenis Layanan</option>
                  <option value="Bridal Makeup">Bridal Makeup</option>
                  <option value="Party Makeup">Party Makeup</option>
                  <option value="Photoshoot Makeup">Photoshoot Makeup</option>
                  <option value="Lainnya">Lainnya</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Deskripsi Tambahan
                </label>
                <textarea
                  value={bookingData.description}
                  onChange={(e) => setBookingData(prev => ({ ...prev, description: e.target.value }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                  rows={3}
                  placeholder="Ceritakan tentang acara Anda..."
                />
              </div>

              <div className="flex gap-3">
                <button
                  onClick={() => setShowBookingModal(false)}
                  className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors"
                >
                  Batal
                </button>
                <button
                  onClick={handleBookingSubmit}
                  disabled={createBookingMutation.isPending}
                  className="flex-1 bg-pink-600 text-white px-4 py-2 rounded-md hover:bg-pink-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {createBookingMutation.isPending ? (
                    <>
                      <Loader2 className="w-4 h-4 inline mr-2 animate-spin" />
                      Menyimpan...
                    </>
                  ) : (
                    <>
                      <Send className="w-4 h-4 inline mr-2" />
                      Kirim Pesanan
                    </>
                  )}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Booking Success Modal */}
      {showSuccessModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-md w-full p-6">
            <div className="text-center">
              <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Send className="w-8 h-8 text-green-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-2">
                Pesanan Berhasil Dikirim!
              </h3>
              <p className="text-gray-600 mb-6">
                Pesanan Anda telah berhasil dikirim ke {mua?.user.full_name}. MUA akan mengonfirmasi pesanan Anda dalam waktu 24 jam.
              </p>
              <div className="bg-gray-50 rounded-lg p-4 mb-6">
                <h4 className="font-medium text-gray-900 mb-2">Detail Pesanan:</h4>
                <div className="text-left space-y-1 text-sm text-gray-600">
                  <p><strong>Layanan:</strong> {bookingData.service_type}</p>
                  <p><strong>Tanggal:</strong> {new Date(bookingData.event_date).toLocaleDateString('id-ID')}</p>
                  <p><strong>Waktu:</strong> {new Date(bookingData.event_date).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' })}</p>
                  <p><strong>Lokasi:</strong> {bookingData.event_location}</p>
                  <p><strong>Durasi:</strong> {bookingData.duration_hours} jam</p>
                </div>
              </div>
              <div className="flex gap-3">
                <button
                  onClick={() => setShowSuccessModal(false)}
                  className="flex-1 px-4 py-2 bg-pink-600 text-white rounded-md hover:bg-pink-700 transition-colors"
                >
                  OK
                </button>
                <button
                  onClick={() => {
                    setShowSuccessModal(false);
                    navigate('/dashboard');
                  }}
                  className="flex-1 px-4 py-2 border border-pink-600 text-pink-600 rounded-md hover:bg-pink-50 transition-colors"
                >
                  Lihat Dashboard
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default MuaProfile;