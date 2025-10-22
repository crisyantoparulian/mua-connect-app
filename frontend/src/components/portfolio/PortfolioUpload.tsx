import { useState } from 'react';
import { muasApi } from '@/api/muas';
import { Upload, X, Camera, Plus, Loader2 } from 'lucide-react';

interface PortfolioUploadProps {
  onSuccess?: () => void;
}

const PortfolioUpload = ({ onSuccess }: PortfolioUploadProps) => {
  const [isOpen, setIsOpen] = useState(false);
  const [file, setFile] = useState<File | null>(null);
  const [preview, setPreview] = useState<string>('');
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [serviceType, setServiceType] = useState('');
  const [isUploading, setIsUploading] = useState(false);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFile = e.target.files?.[0];
    if (selectedFile) {
      // Validate file type
      if (!selectedFile.type.startsWith('image/')) {
        alert('Please select an image file');
        return;
      }

      // Validate file size (10MB max)
      if (selectedFile.size > 10 * 1024 * 1024) {
        alert('File size must be less than 10MB');
        return;
      }

      setFile(selectedFile);

      // Create preview
      const reader = new FileReader();
      reader.onload = (e) => {
        setPreview(e.target?.result as string);
      };
      reader.readAsDataURL(selectedFile);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!file || !title.trim()) {
      alert('Please select an image and enter a title');
      return;
    }

    setIsUploading(true);

    try {
      await muasApi.uploadPortfolioImage(
        file,
        title.trim(),
        description.trim() || undefined,
        serviceType || undefined
      );

      // Reset form
      setFile(null);
      setPreview('');
      setTitle('');
      setDescription('');
      setServiceType('');
      setIsOpen(false);

      // Notify parent component
      if (onSuccess) {
        onSuccess();
      }

      alert('Portfolio item uploaded successfully!');
    } catch (error) {
      console.error('Upload failed:', error);
      alert('Failed to upload portfolio item. Please try again.');
    } finally {
      setIsUploading(false);
    }
  };

  const resetForm = () => {
    setFile(null);
    setPreview('');
    setTitle('');
    setDescription('');
    setServiceType('');
  };

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="w-full border-2 border-dashed border-gray-300 rounded-lg p-8 hover:border-pink-500 transition-colors group"
      >
        <div className="text-center">
          <Plus className="w-12 h-12 mx-auto mb-4 text-gray-400 group-hover:text-pink-500 transition-colors" />
          <h3 className="text-lg font-medium text-gray-900 mb-2">
            Add Portfolio Item
          </h3>
          <p className="text-gray-600">
            Showcase your best work
          </p>
        </div>
      </button>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-sm p-6">
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-lg font-semibold text-gray-900">
          Add New Portfolio Item
        </h3>
        <button
          onClick={() => {
            setIsOpen(false);
            resetForm();
          }}
          className="text-gray-400 hover:text-gray-600"
        >
          <X className="w-6 h-6" />
        </button>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        {/* Image Upload */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Photo
          </label>
          <div className="flex items-center space-x-6">
            {preview ? (
              <div className="relative">
                <img
                  src={preview}
                  alt="Preview"
                  className="w-32 h-32 object-cover rounded-lg"
                />
                <button
                  type="button"
                  onClick={() => {
                    setFile(null);
                    setPreview('');
                  }}
                  className="absolute -top-2 -right-2 bg-red-500 text-white rounded-full p-1 hover:bg-red-600"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>
            ) : (
              <div className="w-32 h-32 bg-gray-100 rounded-lg flex items-center justify-center">
                <Camera className="w-8 h-8 text-gray-400" />
              </div>
            )}

            <div className="flex-1">
              <input
                type="file"
                accept="image/*"
                onChange={handleFileChange}
                className="hidden"
                id="portfolio-image"
              />
              <label
                htmlFor="portfolio-image"
                className="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 cursor-pointer"
              >
                <Upload className="w-4 h-4 mr-2" />
                Choose Image
              </label>
              <p className="text-xs text-gray-500 mt-2">
                JPG, PNG, GIF up to 10MB
              </p>
            </div>
          </div>
        </div>

        {/* Title */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Title *
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
            placeholder="e.g., Bridal Makeup - Sarah"
            required
          />
        </div>

        {/* Service Type */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Service Type
          </label>
          <select
            value={serviceType}
            onChange={(e) => setServiceType(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
          >
            <option value="">Select Service Type</option>
            <option value="Bridal Makeup">Bridal Makeup</option>
            <option value="Party Makeup">Party Makeup</option>
            <option value="Photoshoot Makeup">Photoshoot Makeup</option>
            <option value="Lainnya">Lainnya</option>
          </select>
        </div>

        {/* Description */}
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Description
          </label>
          <textarea
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={3}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-pink-500"
            placeholder="Describe this look, the occasion, products used, etc."
          />
        </div>

        {/* Action Buttons */}
        <div className="flex gap-3">
          <button
            type="button"
            onClick={() => {
              setIsOpen(false);
              resetForm();
            }}
            className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-md hover:bg-gray-50 transition-colors"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={isUploading || !file || !title.trim()}
            className="flex-1 bg-pink-600 text-white px-4 py-2 rounded-md hover:bg-pink-700 transition-colors disabled:bg-gray-300 disabled:cursor-not-allowed"
          >
            {isUploading ? (
              <>
                <Loader2 className="w-4 h-4 inline mr-2 animate-spin" />
                Uploading...
              </>
            ) : (
              <>
                <Upload className="w-4 h-4 inline mr-2" />
                Upload
              </>
            )}
          </button>
        </div>
      </form>
    </div>
  );
};

export default PortfolioUpload;