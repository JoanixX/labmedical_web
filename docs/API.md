# Documentacion de API - LabMedical

## URL Base

- **Desarrollo**: `http://localhost:3000`
- **Produccion**: `https://labmedical-api.onrender.com`

## Autenticacion

Los endpoints administrativos requieren autenticacion JWT. Incluir el token en el header de Autorizacion:

```
Authorization: Bearer <tu_token_jwt>
```

## Endpoints Publicos

### Verificacion de Salud

```http
GET /health
```

**Respuesta:**

```
OK
```

---

### Listar Productos

```http
GET /api/products
```

**Parametros de Consulta:**

- `category` (opcional): Filtrar por slug de categoria
- `search` (opcional): Buscar en nombre y descripcion
- `page` (opcional, por defecto: 1): Numero de pagina
- `limit` (opcional, por defecto: 20, max: 100): Elementos por pagina

**Respuesta:**

```json
{
  "products": [
    {
      "id": 1,
      "name": "Nombre del Producto",
      "slug": "nombre-del-producto",
      "description": "Descripcion del producto",
      "category_id": 1,
      "specifications": {},
      "image_url": "https://...",
      "additional_images": [],
      "regulatory_info": {},
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "total": 100,
  "page": 1,
  "limit": 20
}
```

---

### Obtener Producto por Slug

```http
GET /api/products/:slug
```

**Respuesta:**

```json
{
  "id": 1,
  "name": "Nombre del Producto",
  "slug": "nombre-del-producto",
  ...
}
```

---

### Listar Categorias

```http
GET /api/categories
```

**Respuesta:**

```json
[
  {
    "id": 1,
    "name": "Nombre de Categoria",
    "slug": "slug-categoria",
    "description": "Descripcion de la categoria",
    "created_at": "2024-01-01T00:00:00Z"
  }
]
```

---

### Enviar Solicitud de Cotizacion

```http
POST /api/quotes
```

**Cuerpo de la Solicitud:**

```json
{
  "company_name": "Nombre de Empresa",
  "company_tax_id": "12345678",
  "contact_name": "Juan Perez",
  "email": "juan@empresa.com",
  "phone": "+51987654321",
  "product_ids": [1, 2, 3],
  "estimated_quantity": "100 unidades",
  "message": "Informacion adicional"
}
```

**Reglas de Validacion:**

- `company_name`: 2-255 caracteres
- `contact_name`: 2-255 caracteres
- `email`: Formato de email valido
- `product_ids`: Al menos 1 producto
- `estimated_quantity`: Max 1000 caracteres
- `message`: Max 2000 caracteres

**Respuesta:**

```json
{
  "success": true,
  "message": "Solicitud de cotizacion enviada exitosamente"
}
```

---

## Endpoints Administrativos

### Login

```http
POST /api/admin/login
```

**Cuerpo de la Solicitud:**

```json
{
  "email": "admin@labmedical.com",
  "password": "admin123"
}
```

**Respuesta:**

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "admin": {
    "id": 1,
    "email": "admin@labmedical.com",
    "name": "Administrador"
  }
}
```

---

### Listar Productos (Admin)

```http
GET /api/admin/products
```

**Headers:**

```
Authorization: Bearer <token>
```

**Parametros de Consulta:**

- `page` (opcional, por defecto: 1)
- `limit` (opcional, por defecto: 50, max: 100)
- `active` (opcional): Filtrar por estado activo

---

### Crear Producto

```http
POST /api/admin/products
```

**Headers:**

```
Authorization: Bearer <token>
Content-Type: application/json
```

**Cuerpo de la Solicitud:**

```json
{
  "name": "Nuevo Producto",
  "slug": "nuevo-producto",
  "description": "Descripcion del producto",
  "category_id": 1,
  "specifications": {
    "peso": "1kg",
    "dimensiones": "10x10x10cm"
  },
  "regulatory_info": {
    "certificacion": "CE",
    "registro": "REG-12345"
  }
}
```

---

### Actualizar Producto

```http
PUT /api/admin/products/:id
```

**Headers:**

```
Authorization: Bearer <token>
Content-Type: application/json
```

**Cuerpo de la Solicitud:** (todos los campos opcionales)

```json
{
  "name": "Nombre Actualizado",
  "description": "Descripcion actualizada",
  "is_active": false
}
```

---

### Eliminar Producto

```http
DELETE /api/admin/products/:id
```

**Headers:**

```
Authorization: Bearer <token>
```

---

### Alternar Estado Activo del Producto

```http
PATCH /api/admin/products/:id/toggle
```

**Headers:**

```
Authorization: Bearer <token>
```

---

### Subir Imagen

```http
POST /api/admin/upload
```

**Headers:**

```
Authorization: Bearer <token>
Content-Type: multipart/form-data
```

**Cuerpo de la Solicitud:**

- `file`: Archivo de imagen (JPG, PNG, WebP, max 5MB)

**Respuesta:**

```json
{
  "url": "https://bucket.s3.amazonaws.com/products/uuid.jpg"
}
```

---

### Listar Cotizaciones

```http
GET /api/admin/quotes
```

**Headers:**

```
Authorization: Bearer <token>
```

**Parametros de Consulta:**

- `status` (opcional): Filtrar por estado (pending, contacted, closed)
- `page` (opcional, por defecto: 1)
- `limit` (opcional, por defecto: 50, max: 100)

---

### Obtener Detalles de Cotizacion

```http
GET /api/admin/quotes/:id
```

**Headers:**

```
Authorization: Bearer <token>
```

---

### Actualizar Estado de Cotizacion

```http
PATCH /api/admin/quotes/:id/status
```

**Headers:**

```
Authorization: Bearer <token>
Content-Type: application/json
```

**Cuerpo de la Solicitud:**

```json
{
  "status": "contacted",
  "notes": "Cliente llamado, esperando respuesta"
}
```

---

## Respuestas de Error

Todos los endpoints pueden devolver respuestas de error en este formato:

```json
{
  "error": "Descripcion del mensaje de error"
}
```

**Codigos de Estado HTTP Comunes:**

- `200 OK`: Exito
- `400 Bad Request`: Entrada invalida
- `401 Unauthorized`: Autenticacion faltante o invalida
- `404 Not Found`: Recurso no encontrado
- `429 Too Many Requests`: Limite de tasa excedido
- `500 Internal Server Error`: Error del servidor

---

## Limitacion de Tasa

Los endpoints publicos tienen limitacion de tasa para prevenir abuso:

- Envios de cotizaciones: 5 solicitudes por hora por IP
- Otros endpoints publicos: 100 solicitudes por minuto por IP

Los endpoints administrativos no tienen limitacion de tasa pero requieren autenticacion.
