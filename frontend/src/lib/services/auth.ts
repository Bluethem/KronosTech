import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

const authClient = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json'
  },
  withCredentials: true
});

export type AuthUser = {
  id_usuario: number;
  nombre: string;
  apellido: string;
  email: string;
  telefono?: string | null;
  avatar_url?: string | null;
};

export type AuthResponse = {
  token: string;
  user: AuthUser;
};

export type RegisterPayload = {
  firstName: string;
  lastName: string;
  email: string;
  phone?: string;
  password: string;
};

export const authService = {
  async login(email: string, password: string): Promise<AuthResponse> {
    const response = await authClient.post<AuthResponse>('/auth/login', { email, password });
    return response.data;
  },

  async register(data: RegisterPayload): Promise<AuthResponse> {
    const response = await authClient.post<AuthResponse>('/auth/register', {
      nombre: data.firstName,
      apellido: data.lastName,
      email: data.email,
      telefono: data.phone || undefined,
      password: data.password
    });
    return response.data;
  }
};
