import apiClient from './client';
import { MuaProfile, SearchMuasParams } from '../types';

export const muasApi = {
  searchMuas: async (params: SearchMuasParams): Promise<MuaProfile[]> => {
    const response = await apiClient.get<MuaProfile[]>('/muas/search', { params });
    return response.data;
  },

  getMuaById: async (id: string): Promise<MuaProfile> => {
    const response = await apiClient.get<MuaProfile>(`/muas/${id}`);
    return response.data;
  },

  createPortfolio: async (portfolioData: any): Promise<any> => {
    const response = await apiClient.post('/muas/portfolio', portfolioData);
    return response.data;
  },
};