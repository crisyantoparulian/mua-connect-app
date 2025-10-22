import apiClient from './client';
import type { MuaProfile, SearchMuasParams } from '../types';
import { compressImage } from '../utils/imageCompression';

interface PortfolioResponse {
  data: any[];
  pagination: {
    current_page: number;
    per_page: number;
    total_items: number;
    total_pages: number;
    has_next_page: boolean;
    has_prev_page: boolean;
  };
}

export const muasApi = {
  searchMuas: async (params: SearchMuasParams): Promise<MuaProfile[]> => {
    const response = await apiClient.get<MuaProfile[]>('/muas/search', { params });
    return response.data;
  },

  getMuaById: async (id: string): Promise<MuaProfile> => {
    const response = await apiClient.get<MuaProfile>(`/muas/${id}`);
    return response.data;
  },

  getMuaPortfolio: async (id: string): Promise<any[]> => {
    const response = await apiClient.get<any[]>(`/muas/${id}/portfolio`);
    return response.data;
  },

  // New endpoint for getting current MUA's portfolio with pagination
  getCurrentMuaPortfolio: async (page: number = 1, limit: number = 10): Promise<PortfolioResponse> => {
    const response = await apiClient.get<PortfolioResponse>('/muas/portfolio', {
      params: { page, limit }
    });
    return response.data;
  },

  createPortfolio: async (portfolioData: any): Promise<any> => {
    const response = await apiClient.post('/muas/portfolio', portfolioData);
    return response.data;
  },

  uploadPortfolioImage: async (file: File, title: string, description?: string, serviceType?: string): Promise<any> => {
    // Convert file to base64
    const base64 = await new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.readAsDataURL(file);
      reader.onload = () => resolve(reader.result as string);
      reader.onerror = reject;
    });

    const portfolioData = {
      title,
      description,
      service_type: serviceType,
      image_base64: base64,
    };

    const response = await apiClient.post('/muas/portfolio', portfolioData);
    return response.data;
  },

  // New presigned URL upload with compression
  getPresignedUploadUrl: async (fileName: string, contentType: string, folder?: string): Promise<{
    presigned_url: string;
    public_url: string;
    expires_in: number;
  }> => {
    const response = await apiClient.post('/muas/upload/presigned', {
      file_name: fileName,
      content_type: contentType,
      folder: folder || 'portfolio'
    });
    return response.data;
  },

  // Upload compressed image directly to S3
  uploadImageDirect: async (
    file: File,
    presignedUrl: string,
    onProgress?: (progress: number) => void
  ): Promise<{ success: boolean; error?: string }> => {
    return new Promise((resolve) => {
      const xhr = new XMLHttpRequest();

      // Compress image before upload
      compressImage(file, (compressedBlob) => {
        if (!compressedBlob) {
          resolve({ success: false, error: 'Failed to compress image' });
          return;
        }

        console.log('DEBUG: Starting upload to:', presignedUrl);
        console.log('DEBUG: Original file type:', file.type);
        console.log('DEBUG: Compressed blob type:', compressedBlob.type);
        console.log('DEBUG: Compressed blob size:', compressedBlob.size);

        xhr.upload.addEventListener('progress', (event) => {
          if (event.lengthComputable && onProgress) {
            const progress = Math.round((event.loaded / event.total) * 100);
            onProgress(progress);
          }
        });

        xhr.addEventListener('load', () => {
          console.log('DEBUG: Upload completed with status:', xhr.status);
          console.log('DEBUG: Response headers:', xhr.getAllResponseHeaders());
          if (xhr.status >= 200 && xhr.status < 300) {
            resolve({ success: true });
          } else {
            console.log('DEBUG: Upload failed. Response text:', xhr.responseText);
            resolve({ success: false, error: `Upload failed with status ${xhr.status}: ${xhr.responseText}` });
          }
        });

        xhr.addEventListener('error', () => {
          console.log('DEBUG: Network error during upload');
          resolve({ success: false, error: 'Upload failed due to network error' });
        });

        xhr.open('PUT', presignedUrl);
        xhr.setRequestHeader('Content-Type', compressedBlob.type || file.type);
        xhr.send(compressedBlob);
      });
    });
  },

  // Upload compressed blob directly to S3 with explicit content type
  uploadImageDirectWithBlob: async (
    blob: Blob,
    presignedUrl: string,
    contentType: string,
    onProgress?: (progress: number) => void
  ): Promise<{ success: boolean; error?: string }> => {
    return new Promise((resolve) => {
      const xhr = new XMLHttpRequest();

      console.log('DEBUG: Starting blob upload to:', presignedUrl);
      console.log('DEBUG: Blob type:', blob.type);
      console.log('DEBUG: Blob size:', blob.size);
      console.log('DEBUG: Content type header:', contentType);

      xhr.upload.addEventListener('progress', (event) => {
        if (event.lengthComputable && onProgress) {
          const progress = Math.round((event.loaded / event.total) * 100);
          onProgress(progress);
        }
      });

      xhr.addEventListener('load', () => {
        console.log('DEBUG: Upload completed with status:', xhr.status);
        console.log('DEBUG: Response headers:', xhr.getAllResponseHeaders());
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve({ success: true });
        } else {
          console.log('DEBUG: Upload failed. Response text:', xhr.responseText);
          resolve({ success: false, error: `Upload failed with status ${xhr.status}: ${xhr.responseText}` });
        }
      });

      xhr.addEventListener('error', () => {
        console.log('DEBUG: Network error during upload');
        resolve({ success: false, error: 'Upload failed due to network error' });
      });

      xhr.open('PUT', presignedUrl);
      xhr.setRequestHeader('Content-Type', contentType);
      xhr.send(blob);
    });
  },
};