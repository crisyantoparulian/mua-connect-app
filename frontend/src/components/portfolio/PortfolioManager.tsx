import { useState, useEffect } from 'react';
import { muasApi } from '@/api/muas';
import { useAuthStore } from '@/store/authStore';
import { Plus, Edit2, Trash2, Image as ImageIcon, X, Loader2, Upload, Camera } from 'lucide-react';
import { validateImageFile, generateCompressedFileName, compressImage } from '@/utils/imageCompression';

interface PortfolioItem {
  id: string;
  mua_id: string;
  title: string;
  description?: string;
  image_url: string;
  service_type?: string;
  created_at: string;
}

interface CreatePortfolioRequest {
  title: string;
  description?: string;
  image_url: string;
  service_type?: string;
}

interface PortfolioFormData {
  title: string;
  description: string;
  image_url: string;
  service_type: string;
  image_file?: File;
  image_preview?: string;
}

const PortfolioManager = () => {
  const { user } = useAuthStore();
  const [portfolioItems, setPortfolioItems] = useState<PortfolioItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [showModal, setShowModal] = useState(false);
  const [editingItem, setEditingItem] = useState<PortfolioItem | null>(null);
  const [submitting, setSubmitting] = useState(false);
  const [formData, setFormData] = useState<PortfolioFormData>({
    title: '',
    description: '',
    image_url: '',
    service_type: '',
  });

  useEffect(() => {
    fetchPortfolioItems();
  }, [user]);

  const fetchPortfolioItems = async () => {
    try {
      setLoading(true);
      console.log('DEBUG: Current user from auth store:', user);

      if (user?.id) {
        console.log('DEBUG: User ID found:', user.id);
        // Use the new endpoint to get current MUA's portfolio with pagination
        const response = await muasApi.getCurrentMuaPortfolio(1, 50); // Get first 50 items
        console.log('DEBUG: Portfolio response fetched:', response);
        setPortfolioItems(response.data);
      } else {
        console.log('DEBUG: No user ID found in auth store');
        setPortfolioItems([]);
      }
    } catch (error) {
      console.error('DEBUG: Failed to fetch portfolio items:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!editingItem && !formData.image_file) {
      alert('Please select an image file');
      return;
    }

    if (!formData.title.trim()) {
      alert('Please enter a title');
      return;
    }

    try {
      setSubmitting(true);

      if (editingItem) {
        // For editing, we would need to implement an update endpoint with file upload
        // For now, let's just show a message
        alert('Edit functionality with file upload coming soon!');
        setShowModal(false);
      } else {
        // Create new portfolio item with presigned URL upload
        const file = formData.image_file!;

        // Validate image file
        const validation = validateImageFile(file);
        if (!validation.valid) {
          alert(validation.error);
          return;
        }

        // First compress the image to determine the final content type
        const compressedBlob = await new Promise<Blob>((resolve) => {
          compressImage(file, (blob) => {
            if (blob) {
              resolve(blob);
            } else {
              resolve(new Blob([file], { type: file.type }));
            }
          });
        });

        // Generate unique filename
        const uniqueFileName = generateCompressedFileName(file.name);
        const finalContentType = compressedBlob.type || file.type;

        console.log('DEBUG: Original file type:', file.type);
        console.log('DEBUG: Final content type for upload:', finalContentType);

        // Get presigned URL from backend with the actual content type that will be used
        console.log('DEBUG: Requesting presigned URL for file:', uniqueFileName);
        const { presigned_url, public_url } = await muasApi.getPresignedUploadUrl(
          uniqueFileName,
          finalContentType,
          'portfolio'
        );
        console.log('DEBUG: Received presigned URL and public URL:', { presigned_url, public_url });

        // Upload compressed image directly to S3
        console.log('DEBUG: Starting direct upload to S3...');
        const uploadResult = await muasApi.uploadImageDirectWithBlob(
          compressedBlob,
          presigned_url,
          finalContentType,
          (progress) => {
            console.log(`DEBUG: Upload progress: ${progress}%`);
          }
        );

        if (!uploadResult.success) {
          throw new Error(uploadResult.error || 'Upload failed');
        }

        console.log('DEBUG: Upload successful, creating portfolio item with public URL:', public_url);

        // Create portfolio item with the public URL
        await muasApi.createPortfolio({
          title: formData.title.trim(),
          description: formData.description.trim() || undefined,
          service_type: formData.service_type || undefined,
          image_url: public_url
        });

        setShowModal(false);
      }

      setEditingItem(null);
      setFormData({
        title: '',
        description: '',
        image_url: '',
        service_type: '',
        image_file: undefined,
        image_preview: undefined
      });
      fetchPortfolioItems();
    } catch (error) {
      console.error('Failed to save portfolio item:', error);
      alert('Failed to save portfolio item. Please try again.');
    } finally {
      setSubmitting(false);
    }
  };

  const handleEdit = (item: PortfolioItem) => {
    setEditingItem(item);
    setFormData({
      title: item.title,
      description: item.description || '',
      image_url: item.image_url,
      service_type: item.service_type || '',
      image_preview: item.image_url,
      image_file: undefined,
    });
    setShowModal(true);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this portfolio item?')) {
      try {
        // Delete functionality would need to be implemented
        alert('Delete functionality coming soon!');
      } catch (error) {
        console.error('Failed to delete portfolio item:', error);
      }
    }
  };

  const openAddModal = () => {
    setEditingItem(null);
    setFormData({
      title: '',
      description: '',
      image_url: '',
      service_type: '',
      image_file: undefined,
      image_preview: undefined
    });
    setShowModal(true);
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      // Validate image file using utility function
      const validation = validateImageFile(file);
      if (!validation.valid) {
        alert(validation.error);
        return;
      }

      setFormData({
        ...formData,
        image_file: file,
        image_preview: URL.createObjectURL(file)
      });
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
    <div>
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-semibold text-gray-900">Kelola Portofolio</h2>
        <button
          onClick={openAddModal}
          className="flex items-center gap-2 px-4 py-2 bg-pink-600 text-white rounded-lg hover:bg-pink-700"
        >
          <Plus className="w-4 h-4" />
          Tambah Portofolio
        </button>
      </div>

      {portfolioItems.length === 0 ? (
        <div className="text-center py-12">
          <ImageIcon className="w-12 h-12 text-gray-400 mx-auto mb-4" />
          <p className="text-gray-500 mb-4">Belum ada item portofolio</p>
          <button
            onClick={openAddModal}
            className="text-pink-600 hover:text-pink-700 font-medium"
          >
            Tambah item pertama
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {portfolioItems.map((item) => (
            <div key={item.id} className="bg-white rounded-lg shadow-sm overflow-hidden">
              <div className="aspect-w-16 aspect-h-12 bg-gray-100">
                <img
                  src={item.image_url}
                  alt={item.title}
                  className="w-full h-48 object-cover"
                  onError={(e) => {
                    (e.target as HTMLImageElement).src = 'https://picsum.photos/seed/portfolio/400/300.jpg';
                  }}
                />
              </div>
              <div className="p-4">
                <h3 className="font-semibold text-gray-900 mb-2">{item.title}</h3>
                {item.description && (
                  <p className="text-sm text-gray-600 mb-2">{item.description}</p>
                )}
                {item.service_type && (
                  <p className="text-xs text-pink-600 font-medium mb-3">{item.service_type}</p>
                )}
                <div className="flex justify-end gap-2">
                  <button
                    onClick={() => handleEdit(item)}
                    className="p-2 text-gray-600 hover:text-pink-600 hover:bg-pink-50 rounded-lg"
                  >
                    <Edit2 className="w-4 h-4" />
                  </button>
                  <button
                    onClick={() => handleDelete(item.id)}
                    className="p-2 text-gray-600 hover:text-red-600 hover:bg-red-50 rounded-lg"
                  >
                    <Trash2 className="w-4 h-4" />
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Modal */}
      {showModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold text-gray-900">
                {editingItem ? 'Edit Portofolio' : 'Tambah Portofolio'}
              </h3>
              <button
                onClick={() => setShowModal(false)}
                className="p-1 hover:bg-gray-100 rounded-lg"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            <form onSubmit={handleSubmit} className="space-y-4">
              {/* Image Upload */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Gambar {!editingItem && '*'}
                </label>
                <div className="space-y-3">
                  {formData.image_preview ? (
                    <div className="relative">
                      <img
                        src={formData.image_preview}
                        alt="Preview"
                        className="w-full h-48 object-cover rounded-lg"
                      />
                      {!editingItem && (
                        <button
                          type="button"
                          onClick={() => setFormData({
                            ...formData,
                            image_file: undefined,
                            image_preview: undefined
                          })}
                          className="absolute top-2 right-2 bg-red-500 text-white rounded-full p-1 hover:bg-red-600"
                        >
                          <X className="w-4 h-4" />
                        </button>
                      )}
                    </div>
                  ) : (
                    <div className="w-full h-48 bg-gray-100 rounded-lg flex items-center justify-center">
                      <Camera className="w-12 h-12 text-gray-400" />
                    </div>
                  )}

                  {!editingItem && (
                    <div>
                      <input
                        type="file"
                        accept="image/*"
                        onChange={handleFileChange}
                        className="hidden"
                        id="portfolio-image-upload"
                      />
                      <label
                        htmlFor="portfolio-image-upload"
                        className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 cursor-pointer"
                      >
                        <Upload className="w-4 h-4 mr-2" />
                        Choose Image
                      </label>
                      <p className="text-xs text-gray-500 mt-1">
                        JPG, PNG, GIF up to 10MB
                      </p>
                    </div>
                  )}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Judul *
                </label>
                <input
                  type="text"
                  required
                  value={formData.title}
                  onChange={(e) => setFormData({ ...formData, title: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-pink-500 focus:border-pink-500"
                  placeholder="e.g., Bridal Makeup - Sarah"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Jenis Layanan
                </label>
                <select
                  value={formData.service_type}
                  onChange={(e) => setFormData({ ...formData, service_type: e.target.value })}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-pink-500 focus:border-pink-500"
                >
                  <option value="">Select Service Type</option>
                  <option value="Bridal Makeup">Bridal Makeup</option>
                  <option value="Party Makeup">Party Makeup</option>
                  <option value="Photoshoot Makeup">Photoshoot Makeup</option>
                  <option value="Lainnya">Lainnya</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Deskripsi
                </label>
                <textarea
                  value={formData.description}
                  onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                  rows={3}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-pink-500 focus:border-pink-500"
                  placeholder="Describe this look, the occasion, products used, etc."
                />
              </div>

              <div className="flex justify-end gap-3 pt-4">
                <button
                  type="button"
                  onClick={() => setShowModal(false)}
                  className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg"
                >
                  Batal
                </button>
                <button
                  type="submit"
                  disabled={submitting}
                  className="px-4 py-2 bg-pink-600 text-white rounded-lg hover:bg-pink-700 disabled:opacity-50"
                >
                  {submitting ? (
                    <Loader2 className="w-4 h-4 animate-spin" />
                  ) : (
                    editingItem ? 'Update' : 'Tambah'
                  )}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default PortfolioManager;