// Store persistente del carrito de cotizacion
// los productos seleccionados se guardan en localStorage
// y se sincronizan entre paginas via nanostores

import { persistentAtom } from "@nanostores/persistent";

export interface QuoteCartItem {
  id: number;
  name: string;
  brand: string;
  slug: string;
}

// el store persistente sobrevive a recargas y navegacion
export const quoteCart = persistentAtom<QuoteCartItem[]>(
  "labmedical_quote_cart",
  [],
  {
    encode: JSON.stringify,
    decode: JSON.parse,
  },
);

// se agrega producto al carrito (evita duplicados)
export function addToCart(item: QuoteCartItem): void {
  const current = quoteCart.get();
  if (!current.find((p) => p.id === item.id)) {
    quoteCart.set([...current, item]);
  }
}

// se quita producto del carrito
export function removeFromCart(id: number): void {
  const current = quoteCart.get();
  quoteCart.set(current.filter((p) => p.id !== id));
}

// se limpia el carrito completo
export function clearCart(): void {
  quoteCart.set([]);
}

// se obtiene la cantidad de productos en el carrito
export function getCartCount(): number {
  return quoteCart.get().length;
}

// se obtienen los IDs para enviar al backend
export function getCartProductIds(): number[] {
  return quoteCart.get().map((p) => p.id);
}