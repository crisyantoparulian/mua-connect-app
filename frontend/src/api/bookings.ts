import apiClient from './client';

export interface CreateBookingRequest {
  mua_id: string;
  service_type: string;
  description?: string;
  event_date: string;
  event_location: string;
  duration_hours: number;
  price: string;
  deposit_amount?: string;
}

export interface Booking {
  id: string;
  customer_id: string;
  mua_id: string;
  service_type: string;
  description?: string;
  event_date: string;
  event_location: string;
  duration_hours: number;
  price: number;
  status: 'pending' | 'confirmed' | 'cancelled' | 'completed' | 'no_show';
  deposit_amount?: number;
  deposit_paid: boolean;
  final_payment_paid: boolean;
  created_at: string;
  updated_at: string;
}

export const bookingsApi = {
  createBooking: async (bookingData: CreateBookingRequest): Promise<Booking> => {
    const response = await apiClient.post('/bookings', bookingData);
    return response.data;
  },

  getBookings: async (): Promise<Booking[]> => {
    const response = await apiClient.get('/bookings');
    return response.data;
  },

  updateBookingStatus: async (bookingId: string, status: string): Promise<Booking> => {
    const response = await apiClient.put(`/bookings/${bookingId}/status`, { status });
    return response.data;
  },
};