import apiClient from './client';

export interface DashboardStats {
  total_bookings: number;
  pending_bookings: number;
  confirmed_bookings: number;
  completed_bookings: number;
  total_revenue: number;
  average_rating?: number;
  total_reviews: number;
  portfolio_items: number;
}

export interface RecentBooking {
  id: string;
  customer_name: string;
  service_type: string;
  event_date: string;
  status: 'pending' | 'confirmed' | 'completed' | 'cancelled';
  price: number;
}

export interface DashboardResponse {
  stats: DashboardStats;
  recent_bookings: RecentBooking[];
  upcoming_bookings: RecentBooking[];
}

// Availability API types
export interface TimeSlot {
  id: string;
  start_time: string;
  end_time: string;
  is_available: boolean;
  recurring?: boolean;
  day_of_week?: number;
  specific_date?: string;
}

export interface CreateAvailabilityRequest {
  start_time: string;
  end_time: string;
  recurring: boolean;
  day_of_week?: number[];
  specific_date?: string;
}

export interface BookingCalendar {
  id: string;
  customer_name: string;
  service_type: string;
  start_time: string;
  end_time: string;
  status: 'pending' | 'confirmed' | 'completed' | 'cancelled';
  customer_phone?: string;
  location?: string;
  notes?: string;
}

export const dashboardApi = {
  getDashboard: async (): Promise<DashboardResponse> => {
    const response = await apiClient.get('/dashboard');
    return response.data;
  },

  updateAvailability: async (isAvailable: boolean): Promise<{ is_available: boolean; message: string }> => {
    const response = await apiClient.put('/dashboard/availability', { is_available: isAvailable });
    return response.data;
  },

  // Availability Management
  getAvailabilitySlots: async (month?: string): Promise<TimeSlot[]> => {
    const params = month ? { month } : {};
    const response = await apiClient.get('/dashboard/availability/slots', { params });
    return response.data;
  },

  createAvailabilitySlot: async (data: CreateAvailabilityRequest): Promise<TimeSlot> => {
    const response = await apiClient.post('/dashboard/availability/slots', data);
    return response.data;
  },

  updateAvailabilitySlot: async (id: string, data: Partial<CreateAvailabilityRequest>): Promise<TimeSlot> => {
    const response = await apiClient.put(`/dashboard/availability/slots/${id}`, data);
    return response.data;
  },

  deleteAvailabilitySlot: async (id: string): Promise<void> => {
    await apiClient.delete(`/dashboard/availability/slots/${id}`);
  },

  // Calendar Bookings - get actual bookings for calendar display
  getCalendarBookings: async (startDate: string, endDate: string): Promise<BookingCalendar[]> => {
    const response = await apiClient.get('/dashboard/calendar/bookings', {
      params: { start_date: startDate, end_date: endDate }
    });
    return response.data;
  },

  updateBookingStatus: async (bookingId: string, status: string): Promise<BookingCalendar> => {
    const response = await apiClient.put(`/dashboard/bookings/${bookingId}/status`, { status });
    return response.data;
  },
};