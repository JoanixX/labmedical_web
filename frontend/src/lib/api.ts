// cliente api para conectar con el backend

const API_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000';

export interface Product {
  id: number;
  name: string;
  slug: string;
  description: string;
  category_id: number;
  specifications: Record<string, any>;
  image_url: string | null;
  additional_images: string[];
  regulatory_info: Record<string, any>;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Category {
  id: number;
  name: string;
  slug: string;
  description: string | null;
  created_at: string;
}

export interface ProductsResponse {
  products: Product[];
  total: number;
  page: number;
  limit: number;
}

export async function getProducts(params?: {
  category?: string;
  search?: string;
  page?: number;
  limit?: number;
}): Promise<ProductsResponse> {
  const queryParams = new URLSearchParams();
  if (params?.category) queryParams.set('category', params.category);
  if (params?.search) queryParams.set('search', params.search);
  if (params?.page) queryParams.set('page', params.page.toString());
  if (params?.limit) queryParams.set('limit', params.limit.toString());

  const url = `${API_URL}/api/products${queryParams.toString() ? '?' + queryParams.toString() : ''}`;
  const response = await fetch(url);
  
  if (!response.ok) {
    throw new Error('error al cargar productos');
  }
  
  return response.json();
}

export async function getProductBySlug(slug: string): Promise<Product> {
  const response = await fetch(`${API_URL}/api/products/${slug}`);
  
  if (!response.ok) {
    throw new Error('producto no encontrado');
  }
  
  return response.json();
}

export async function getCategories(): Promise<Category[]> {
  const response = await fetch(`${API_URL}/api/categories`);
  
  if (!response.ok) {
    throw new Error('error al cargar categorias');
  }
  
  return response.json();
}

export async function submitQuote(data: {
  company_name: string;
  company_tax_id: string;
  contact_name: string;
  email: string;
  phone: string;
  product_ids: number[];
  estimated_quantity: string;
  message?: string;
}): Promise<{ success: boolean; message: string }> {
  const response = await fetch(`${API_URL}/api/quotes`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });
  
  if (!response.ok) {
    throw new Error('error al enviar cotizacion');
  }
  
  return response.json();
}
