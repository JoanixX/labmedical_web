# LabMedical - Plataforma de Catalogo Medico B2B

Plataforma profesional B2B de catalogo para LabMedical, proveedor de productos medicos. Esta plataforma sirve como catalogo digital con sistema de cotizaciones para clientes empresariales.

## Estructura del Proyecto

```
labmedical_web/
├── backend/          # API REST en Rust/Axum
│   ├── src/          # Codigo fuente del servidor
│   ├── migrations/   # Migraciones de base de datos
│   └── .env.example  # Variables de entorno
├── frontend/         # Aplicacion web Astro/TypeScript
│   ├── src/          # Codigo fuente del frontend
│   │   ├── components/  # Componentes reutilizables
│   │   ├── layouts/     # Layouts base
│   │   ├── pages/       # Paginas de la aplicacion
│   │   └── lib/         # Cliente API y utilidades
│   └── .env.example  # Variables de entorno
├── docs/             # Documentacion del proyecto
└── README.md
```

## Stack Tecnologico

### Backend

- **Lenguaje**: Rust
- **Framework**: Axum 0.7
- **Base de Datos**: PostgreSQL
- **Autenticacion**: JWT + bcrypt
- **Almacenamiento**: AWS S3
- **Email**: API Resend
- **Hosting**: Render.com

### Frontend

- **Framework**: Astro 4.x
- **Lenguaje**: TypeScript
- **Estilos**: CSS vanilla
- **Hosting**: Vercel

## Funcionalidades

### Funcionalidades Publicas

- Pagina de inicio con informacion de la empresa
- Catalogo de productos con busqueda y filtros por categoria
- Paginas de detalle de productos con especificaciones
- Navegacion por categorias
- Formulario de solicitud de cotizaciones con validacion

### Funcionalidades Administrativas

- Autenticacion segura
- Gestion de productos (CRUD)
- Gestion de categorias
- Bandeja de entrada y gestion de cotizaciones
- Carga de imagenes a S3
- Panel de control con metricas

## Modelo de Negocio

Esta es una **plataforma B2B**, no un sitio de comercio electronico:

- Sin pagos directos ni carrito de compras
- Los clientes navegan el catalogo y solicitan cotizaciones
- El equipo de ventas maneja las negociaciones offline
- Enfoque en presentacion profesional y generacion de leads

## Comenzando

### Prerequisitos

- Rust 1.70+ (para backend)
- Node.js 18+ (para frontend)
- PostgreSQL 14+
- Cuenta AWS S3
- Cuenta Resend para emails

### Configuracion del Backend

Ver [backend/README.md](./backend/README.md) para instrucciones detalladas.

Inicio rapido:

```bash
cd backend
cp .env.example .env
# editar .env con tus credenciales
cargo install sqlx-cli
sqlx migrate run
cargo run
```

El backend estara disponible en `http://localhost:3000`

### Configuracion del Frontend

Ver [frontend/README.md](./frontend/README.md) para instrucciones detalladas.

Inicio rapido:

```bash
cd frontend
cp .env.example .env
# editar .env si es necesario (por defecto apunta a localhost:3000)
npm install
npm run dev
```

El frontend estara disponible en `http://localhost:4321`

### Ejecucion Local Completa

Para ejecutar el proyecto completo localmente:

1. **Iniciar PostgreSQL** (asegurate de que este corriendo)

2. **Terminal 1 - Backend**:

```bash
cd backend
cargo run
```

3. **Terminal 2 - Frontend**:

```bash
cd frontend
npm run dev
```

4. Abrir navegador en `http://localhost:4321`

## Documentacion

- [Documentacion de API](./docs/API.md)
- [README Backend](./backend/README.md)
- [README Frontend](./frontend/README.md)

## Acceso de Administrador por Defecto

- Email: `admin@labmedical.com`
- Contraseña: `admin123`

**⚠️ Cambiar inmediatamente en produccion**

## Deployment

### Backend (Render.com)

- Tier gratuito con PostgreSQL incluido
- Deployment automatico desde GitHub
- Ver `backend/render.yaml` para configuracion

### Frontend (Vercel)

- Tier gratuito con CDN global
- Deployment automatico desde GitHub
- Configuracion proximamente

## Caracteristicas de Seguridad

- Autenticacion JWT con expiracion de 24h
- Hashing de contraseñas con bcrypt
- Validacion de entrada (cliente + servidor)
- Rate limiting en endpoints publicos
- Proteccion contra inyeccion SQL
- Configuracion CORS
- Validacion de carga de archivos

## Esquema de Base de Datos

- **products**: Catalogo de productos con especificaciones JSONB
- **categories**: Categorizacion de productos
- **quotes**: Solicitudes de cotizacion de clientes
- **admins**: Usuarios administradores con contraseñas hasheadas

## Endpoints de API

### Publicos

- `GET /api/products` - Listar productos
- `GET /api/products/:slug` - Detalles de producto
- `GET /api/categories` - Listar categorias
- `POST /api/quotes` - Enviar solicitud de cotizacion

### Administrador (requiere JWT)

- `POST /api/admin/login` - Autenticacion
- CRUD completo para productos, categorias y cotizaciones
- `POST /api/admin/upload` - Carga de imagenes

## Licencia

Propietario - LabMedical

## Contacto

Para preguntas o soporte, contactar: alvaradocjosorio@gmail.com

---

Construido para LabMedical
