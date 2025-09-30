export interface User {
  id: string;
  email: string;
  user_type: 'customer' | 'mua';
  full_name: string;
  phone_number?: string;
  profile_picture_url?: string;
  is_verified: boolean;
  created_at: string;
}

export interface MuaProfile {
  id: string;
  user: User;
  bio?: string;
  experience_years?: number;
  specialization?: string[];
  location: string;
  latitude?: number;
  longitude?: number;
  is_available: boolean;
  average_rating?: number;
  total_reviews?: number;
  created_at: string;
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

export interface PortfolioItem {
  id: string;
  mua_id: string;
  title: string;
  description?: string;
  image_url: string;
  service_type?: string;
  created_at: string;
}

export interface Review {
  id: string;
  booking_id: string;
  reviewer_id: string;
  reviewee_id: string;
  rating: number;
  comment?: string;
  created_at: string;
}

export interface AuthResponse {
  user: User;
  access_token: string;
  token_type: string;
  expires_in: number;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  email: string;
  password: string;
  user_type: 'customer' | 'mua';
  full_name: string;
  phone_number?: string;
}

export interface CreateBookingRequest {
  mua_id: string;
  service_type: string;
  description?: string;
  event_date: string;
  event_location: string;
  duration_hours: number;
  price: number;
  deposit_amount?: number;
}

export interface SearchMuasParams {
  location?: string;
  latitude?: number;
  longitude?: number;
  radius?: number;
  date?: string;
  specialization?: string;
  min_rating?: number;
  page?: number;
  limit?: number;
}