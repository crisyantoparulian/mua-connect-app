import apiClient from './client';

export interface PortfolioItem {
  id: string;
  mua_id: string;
  title: string;
  description?: string;
  image_url: string;
  service_type?: string;
  created_at: string;
}

export interface CreatePortfolioRequest {
  title: string;
  description?: string;
  image_url: string;
  service_type?: string;
}

export interface UpdatePortfolioRequest {
  title?: string;
  description?: string;
  image_url?: string;
  service_type?: string;
}

export const portfolioApi = {
  getPortfolioItems: async (): Promise<PortfolioItem[]> => {
    const response = await apiClient.get('/portfolio');
    return response.data;
  },

  createPortfolioItem: async (data: CreatePortfolioRequest): Promise<PortfolioItem> => {
    const response = await apiClient.post('/portfolio', data);
    return response.data;
  },

  updatePortfolioItem: async (id: string, data: UpdatePortfolioRequest): Promise<PortfolioItem> => {
    const response = await apiClient.put(`/portfolio/${id}`, data);
    return response.data;
  },

  deletePortfolioItem: async (id: string): Promise<{ message: string }> => {
    const response = await apiClient.delete(`/portfolio/${id}`);
    return response.data;
  },
};