import { Link } from 'react-router-dom';
import Button from '@/components/ui/Button';
import { Search, Star, Calendar, Users } from 'lucide-react';

const Home = () => {
  return (
    <div className="min-h-screen bg-gradient-to-br from-pink-50 to-purple-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Hero Section */}
        <div className="pt-20 pb-16 text-center">
          <h1 className="text-5xl font-bold text-gray-900 mb-6">
            Temukan Makeup Artist
            <span className="text-pink-600 block">Profesional</span>
          </h1>
          <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
            Platform marketplace terpercaya yang menghubungkan Anda dengan makeup artist
            berpengalaman untuk setiap acara spesial Anda.
          </p>
          <div className="flex justify-center space-x-4">
            <Link to="/search">
              <Button size="lg" className="bg-pink-600 hover:bg-pink-700">
                <Search className="w-5 h-5 mr-2" />
                Cari MUA
              </Button>
            </Link>
            <Link to="/register">
              <Button variant="outline" size="lg">
                Daftar sebagai MUA
              </Button>
            </Link>
          </div>
        </div>

        {/* Features Section */}
        <div className="py-16">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-gray-900 mb-4">
              Mengapa Memilih MUA Connect?
            </h2>
            <p className="text-lg text-gray-600">
              Platform yang dirancang khusus untuk memenuhi kebutuhan kecantikan Anda
            </p>
          </div>

          <div className="grid md:grid-cols-3 gap-8">
            <div className="bg-white p-8 rounded-lg shadow-sm text-center">
              <div className="w-16 h-16 bg-pink-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Search className="w-8 h-8 text-pink-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-2">
                Pencarian Mudah
              </h3>
              <p className="text-gray-600">
                Temukan MUA terdekat dengan filter berdasarkan lokasi, spesialisasi, dan rating
              </p>
            </div>

            <div className="bg-white p-8 rounded-lg shadow-sm text-center">
              <div className="w-16 h-16 bg-purple-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Calendar className="w-8 h-8 text-purple-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-2">
                Booking Online
              </h3>
              <p className="text-gray-600">
                Pesan jasa MUA secara online dengan sistem pembayaran aman dan terpercaya
              </p>
            </div>

            <div className="bg-white p-8 rounded-lg shadow-sm text-center">
              <div className="w-16 h-16 bg-indigo-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <Star className="w-8 h-8 text-indigo-600" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-2">
                Rating & Review
              </h3>
              <p className="text-gray-600">
                Lihat rating dan review dari pelanggan lain untuk memastikan kualitas layanan
              </p>
            </div>
          </div>
        </div>

        {/* CTA Section */}
        <div className="py-16 bg-gradient-to-r from-pink-600 to-purple-600 rounded-lg text-center text-white">
          <h2 className="text-3xl font-bold mb-4">
            Siap untuk Tampil Memukau?
          </h2>
          <p className="text-lg mb-8 opacity-90">
            Bergabunglah dengan ribuan pelanggan yang telah merasakan layanan terbaik kami
          </p>
          <div className="flex justify-center space-x-4">
            <Link to="/search">
              <Button variant="outline" size="lg" className="border-white text-white hover:bg-white hover:text-pink-600">
                Mulai Pencarian
              </Button>
            </Link>
            <Link to="/register">
              <Button size="lg" className="bg-white text-pink-600 hover:bg-gray-100">
                Daftar Sekarang
              </Button>
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Home;