/**
 * Image compression utilities for direct S3 uploads
 */

// Image compression with quality setting (0.1 to 1.0)
const MAX_WIDTH = 1920; // Maximum width for compressed images
const MAX_HEIGHT = 1920; // Maximum height for compressed images
const QUALITY = 0.8; // Compression quality (0.1 to 1.0)

/**
 * Compress an image file using canvas API
 * @param file - The original file
 * @param callback - Callback function with compressed blob
 */
export const compressImage = (file: File, callback: (blob: Blob | null) => void) => {
  const reader = new FileReader();

  reader.onload = (e) => {
    const img = new Image();

    img.onload = () => {
      // Create canvas for compression
      const canvas = document.createElement('canvas');
      const ctx = canvas.getContext('2d');

      // Calculate new dimensions while maintaining aspect ratio
      let { width, height } = calculateDimensions(img.width, img.height);

      canvas.width = width;
      canvas.height = height;

      // Draw and compress image
      ctx.drawImage(img, 0, 0, width, height);

      // Convert to blob with specified quality
      canvas.toBlob(
        (blob) => {
          callback(blob);
        },
        file.type,
        QUALITY
      );
    };

    img.src = e.target?.result as string;
  };

  reader.readAsDataURL(file);
};

/**
 * Calculate new dimensions for image while maintaining aspect ratio
 * @param originalWidth
 * @param originalHeight
 */
const calculateDimensions = (originalWidth: number, originalHeight: number) => {
  let { width, height } = { width: originalWidth, height: originalHeight };

  // If image is larger than maximum dimensions, scale it down
  if (width > MAX_WIDTH || height > MAX_HEIGHT) {
    const aspectRatio = width / height;

    if (width > height) {
      width = MAX_WIDTH;
      height = Math.round(width / aspectRatio);
    } else {
      height = MAX_HEIGHT;
      width = Math.round(height * aspectRatio);
    }
  }

  return { width, height };
};

/**
 * Generate a unique filename for the compressed image
 * @param originalName - Original filename
 */
export const generateCompressedFileName = (originalName: string): string => {
  const nameWithoutExt = originalName.substring(0, originalName.lastIndexOf('.')) || originalName;
  const extension = originalName.substring(originalName.lastIndexOf('.')) || '.jpg';
  const timestamp = Date.now();

  return `${nameWithoutExt}_compressed_${timestamp}${extension}`;
};

/**
 * Validate image file before upload
 * @param file - File to validate
 * @returns Validation result
 */
export const validateImageFile = (file: File): { valid: boolean; error?: string } => {
  // Check file type
  const allowedTypes = ['image/jpeg', 'image/jpg', 'image/png', 'image/gif', 'image/webp'];
  if (!allowedTypes.includes(file.type)) {
    return {
      valid: false,
      error: 'Invalid file type. Only JPEG, PNG, GIF, and WebP images are allowed.'
    };
  }

  // Check file size (max 10MB)
  const maxSize = 10 * 1024 * 1024; // 10MB
  if (file.size > maxSize) {
    return {
      valid: false,
      error: 'File size must be less than 10MB.'
    };
  }

  return { valid: true };
};