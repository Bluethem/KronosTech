import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

const apiClient = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

export interface Familia {
  id_familia: number;
  nombre: string;
  descripcion: string | null;
  icono: string | null;
  slug: string | null;
  total_productos: number | null;
}

export interface Categoria {
  id_categoria: number;
  nombre: string;
  descripcion: string | null;
  icono: string | null;
  slug: string | null;
  id_familia: number;
  familia_nombre: string;
  total_productos: number | null;
}

export interface Subcategoria {
  id_subcategoria: number;
  nombre: string;
  descripcion: string | null;
  slug: string | null;
  id_categoria: number;
  orden: number;
  estado: string;
}

export interface Producto {
  id_producto_detalle: number;
  nombre: string;
  sku: string;
  slug: string | null;
  marca: string;
  precio_venta: number;
  precio_base: number;
  descuento_porcentaje: number | null;
  imagen_principal: string | null;
  es_destacado: boolean;
  es_nuevo: boolean;
  es_oferta: boolean;
  stock_disponible: number;
  valoracion_promedio: number | null;
  total_valoraciones: number;
  categoria: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T;
  message?: string;
}

export interface PaginatedResponse<T> {
  success: boolean;
  data: T[];
  pagination: {
    total: number;
    limit: number;
    offset: number;
    total_pages: number;
    current_page: number;
  };
}

export interface ProductosFiltros {
  search?: string;
  categoria?: number;
  subcategoria?: number;
  marca?: number;
  familia?: number;
  precio_min?: number;
  precio_max?: number;
  destacados?: boolean;
  nuevos?: boolean;
  ofertas?: boolean;
  en_stock?: boolean;
  order_by?: string;
  limit?: number;
  offset?: number;
}

export interface ProductoDetalle extends Producto {
  descripcion?: string; // Descripción del producto_detalle (instancia específica)
  producto_descripcion?: string; // Descripción del producto (tipo/modelo general)
  producto_nombre?: string; // Nombre del tipo de producto
  modelo?: string;
  marca_logo?: string;
  imagenes?: string[] | null;
  especificaciones_base?: Record<string, any>;
  peso?: number;
  dimensiones?: string;
  garantia_meses?: number;
  categoria_nombre?: string;
  subcategoria_nombre?: string;
}

export interface Valoracion {
  id_valoracion: number;
  id_producto: number;
  id_usuario: number;
  id_producto_detalle?: number;
  calificacion: number; // 1-5
  titulo?: string;
  comentario?: string;
  compra_verificada: boolean;
  votos_util: number;
  votos_no_util: number;
  aprobado: boolean;
  fecha_creacion: string;
  usuario_nombre?: string;
  usuario_apellido?: string;
  imagenes?: string[];
}

export const catalogoService = {
  async getFamilias(): Promise<Familia[]> {
    const response = await apiClient.get<ApiResponse<Familia[]>>('/familias');
    return response.data.data;
  },

  async getCategorias(idFamilia?: number): Promise<Categoria[]> {
    const response = await apiClient.get<ApiResponse<Categoria[]>>('/categorias', {
      params: idFamilia ? { familia: idFamilia } : {}
    });
    return response.data.data;
  },

  async getSubcategorias(idCategoria?: number): Promise<Subcategoria[]> {
    const response = await apiClient.get<ApiResponse<Subcategoria[]>>('/subcategorias', {
      params: idCategoria ? { categoria: idCategoria } : {}
    });
    return response.data.data;
  },

  async getMarcas(): Promise<any[]> {
    const response = await apiClient.get<ApiResponse<any[]>>('/marcas');
    return response.data.data;
  },

  async getProductos(filtros: ProductosFiltros = {}): Promise<PaginatedResponse<Producto>> {
    const response = await apiClient.get<PaginatedResponse<Producto>>('/productos', {
      params: filtros
    });
    return response.data;
  },

  async getProductoDetalle(id: number): Promise<ProductoDetalle> {
    const response = await apiClient.get<ApiResponse<ProductoDetalle>>(`/productos/${id}`);
    return response.data.data;
  },

  async getProductoDetalleBySlug(slug: string): Promise<ProductoDetalle> {
    const response = await apiClient.get<ApiResponse<ProductoDetalle>>(`/productos/slug/${slug}`);
    return response.data.data;
  },

  async getProductosDestacados(limit = 8): Promise<Producto[]> {
    const response = await apiClient.get<PaginatedResponse<Producto>>('/productos', {
      params: { destacados: true, limit, en_stock: true }
    });
    return response.data.data;
  },

  async getProductosOfertas(limit = 8): Promise<Producto[]> {
    const response = await apiClient.get<PaginatedResponse<Producto>>('/productos', {
      params: { ofertas: true, limit, en_stock: true }
    });
    return response.data.data;
  },

  async getProductosNuevos(limit = 8): Promise<Producto[]> {
    const response = await apiClient.get<PaginatedResponse<Producto>>('/productos', {
      params: { nuevos: true, limit, en_stock: true }
    });
    return response.data.data;
  },

  async getValoraciones(idProducto: number): Promise<Valoracion[]> {
    const response = await apiClient.get<ApiResponse<Valoracion[]>>(`/productos/${idProducto}/valoraciones`);
    return response.data.data;
  },
};
