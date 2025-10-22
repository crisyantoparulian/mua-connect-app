import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { muasApi } from '@/api/muas';
import type { MuaProfile, SearchMuasParams } from '@/types';
import { formatCurrency } from '@/lib/utils';
import { Star, MapPin, Calendar, Clock, User, Search as SearchIcon } from 'lucide-react';

const Search = () => {
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useState<SearchMuasParams>({
    location: '',
    specialization: '',
    min_rating: 0,
    page: 1,
    limit: 12,
  });

  const { data: muas, isLoading, error } = useQuery({
    queryKey: ['muas', searchParams],
    queryFn: () => muasApi.searchMuas(searchParams),
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
                <option value="Bridal Makeup">Bridal Makeup</option>
                <option value="Party Makeup">Party Makeup</option>
                <option value="Photoshoot Makeup">Photoshoot Makeup</option>
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
            <div className="flex items-end gap-2">
              <button
                type="submit"
                className="flex-1 bg-pink-600 text-white py-2 px-4 rounded-md hover:bg-pink-700 transition-colors"
              >
                Cari
              </button>
              <button
                type="button"
                onClick={() => setSearchParams({
                  location: '',
                  specialization: '',
                  min_rating: 0,
                  page: 1,
                  limit: 12,
                })}
                className="px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors"
              >
                Reset
              </button>
            </div>
          </form>
        </div>

        {/* Results Count */}
        {muas && muas.length > 0 && (
          <div className="mb-6 text-gray-600">
            <p>Menemukan {muas.length} MUA</p>
          </div>
        )}

        {/* Error Display */}
        {error && (
          <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-6">
            <p className="font-medium">Terjadi kesalahan</p>
            <p className="text-sm">Gagal memuat data MUA. Silakan coba lagi.</p>
          </div>
        )}

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
                      <User className="w-6 h-6 text-pink-600" />
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
                      <span>{mua.location || 'Lokasi tidak tersedia'}</span>
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
                    <button
                      onClick={() => navigate(`/mua/${mua.id}`)}
                      className="text-pink-600 hover:text-pink-700 font-medium text-sm"
                    >
                      Lihat Profil
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : searchParams.location || searchParams.specialization ? (
          <div className="text-center py-12">
            <div className="text-gray-400 mb-4">
              <SearchIcon className="w-16 h-16 mx-auto" />
            </div>
            <h3 className="text-lg font-medium text-gray-900 mb-2">
              Tidak ada hasil ditemukan
            </h3>
            <p className="text-gray-600">
              Coba ubah kriteria pencarian Anda atau cari dengan kata kunci yang berbeda
            </p>
          </div>
        ) : (
          <div className="text-center py-12">
            <div className="text-gray-400 mb-4">
              <SearchIcon className="w-16 h-16 mx-auto" />
            </div>
            <h3 className="text-lg font-medium text-gray-900 mb-2">
              Cari Makeup Artist
            </h3>
            <p className="text-gray-600">
              Masukkan lokasi atau pilih spesialisasi untuk memulai pencarian
            </p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Search;