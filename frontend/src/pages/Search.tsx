import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { muasApi } from '@/api/muas';
import { MuaProfile, SearchMuasParams } from '@/types';
import { formatCurrency } from '@/lib/utils';
import { Star, MapPin, Calendar, Clock } from 'lucide-react';

const Search = () => {
  const [searchParams, setSearchParams] = useState<SearchMuasParams>({
    location: '',
    specialization: '',
    min_rating: 0,
    page: 1,
    limit: 12,
  });

  const { data: muas, isLoading } = useQuery({
    queryKey: ['muas', searchParams],
    queryFn: () => muasApi.searchMuas(searchParams),
    enabled: !!searchParams.location || !!searchParams.specialization,
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    setSearchParams(prev => ({ ...prev, page: 1 }));
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">
            Cari Makeup Artist
          </h1>
          <p className="text-gray-600">
            Temukan MUA profesional terdekat untuk acara spesial Anda
          </p>
        </div>

        {/* Search Form */}
        <div className="bg-white rounded-lg shadow-sm p-6 mb-8">
          <form onSubmit={handleSearch} className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Lokasi
              </label>
              <input
                type="text"
                placeholder="Kota atau alamat"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                value={searchParams.location || ''}
                onChange={(e) => setSearchParams(prev => ({
                  ...prev,
                  location: e.target.value
                }))}
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Spesialisasi
              </label>
              <select
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                value={searchParams.specialization || ''}
                onChange={(e) => setSearchParams(prev => ({
                  ...prev,
                  specialization: e.target.value
                }))}
              >
                <option value="">Semua</option>
                <option value="bridal">Bridal</option>
                <option value="party">Party</option>
                <option value="photoshoot">Photoshoot</option>
                <option value="graduation">Graduation</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Rating Minimal
              </label>
              <select
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
                value={searchParams.min_rating || 0}
                onChange={(e) => setSearchParams(prev => ({
                  ...prev,
                  min_rating: Number(e.target.value)
                }))}
              >
                <option value={0}>Semua</option>
                <option value={4}>4+ Bintang</option>
                <option value={4.5}>4.5+ Bintang</option>
              </select>
            </div>
            <div className="flex items-end">
              <button
                type="submit"
                className="w-full bg-pink-600 text-white py-2 px-4 rounded-md hover:bg-pink-700 transition-colors"
              >
                Cari
              </button>
            </div>
          </form>
        </div>

        {/* Results */}
        {isLoading ? (
          <div className="text-center py-12">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-pink-600"></div>
            <p className="mt-2 text-gray-600">Memuat hasil pencarian...</p>
          </div>
        ) : muas && muas.length > 0 ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {muas.map((mua) => (
              <div key={mua.id} className="bg-white rounded-lg shadow-sm overflow-hidden hover:shadow-md transition-shadow">
                <div className="p-6">
                  <div className="flex items-center mb-4">
                    <div className="w-12 h-12 bg-pink-100 rounded-full flex items-center justify-center">
                      <span className="text-pink-600 font-semibold">
                        {mua.user.full_name.charAt(0)}
                      </span>
                    </div>
                    <div className="ml-4">
                      <h3 className="font-semibold text-gray-900">
                        {mua.user.full_name}
                      </h3>
                      <div className="flex items-center text-sm text-gray-600">
                        <Star className="w-4 h-4 text-yellow-400 fill-current mr-1" />
                        <span>{mua.average_rating || '0.0'}</span>
                        <span className="mx-1">•</span>
                        <span>{mua.total_reviews || 0} review</span>
                      </div>
                    </div>
                  </div>

                  <div className="space-y-2 text-sm text-gray-600 mb-4">
                    <div className="flex items-center">
                      <MapPin className="w-4 h-4 mr-2" />
                      <span>{mua.location}</span>
                    </div>
                    {mua.experience_years && (
                      <div className="flex items-center">
                        <Clock className="w-4 h-4 mr-2" />
                        <span>{mua.experience_years} tahun pengalaman</span>
                      </div>
                    )}
                    {mua.specialization && mua.specialization.length > 0 && (
                      <div className="flex flex-wrap gap-1 mt-2">
                        {mua.specialization.map((spec, index) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-pink-100 text-pink-700 rounded-full text-xs"
                          >
                            {spec}
                          </span>
                        ))}
                      </div>
                    )}
                  </div>

                  {mua.bio && (
                    <p className="text-sm text-gray-600 mb-4 line-clamp-2">
                      {mua.bio}
                    </p>
                  )}

                  <div className="flex items-center justify-between">
                    <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                      mua.is_available
                        ? 'bg-green-100 text-green-800'
                        : 'bg-red-100 text-red-800'
                    }`}>
                      {mua.is_available ? 'Tersedia' : 'Tidak Tersedia'}
                    </span>
                    <button className="text-pink-600 hover:text-pink-700 font-medium text-sm">
                      Lihat Profil
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="text-center py-12">
            <div className="text-gray-400 mb-4">
              <Calendar className="w-16 h-16 mx-auto" />
            </div>
            <h3 className="text-lg font-medium text-gray-900 mb-2">
              Tidak ada hasil ditemukan
            </h3>
            <p className="text-gray-600">
              Coba ubah kriteria pencarian Anda
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Search;