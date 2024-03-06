# URL Shortener
A simple URL shortener using Rocket and Diesel.

# Setup
1. Install Docker and Docker compose

2. Clone the repository
```bash
git clone https://github.com/peeeuzin/url-shortener
```

1. Create a `.env.local` file in the root of the project. See [.env.example](./.env.example) to see the required environment variables.

2. Run the application
```bash
docker compose -f docker-compose.dev.yml up
```

to run the application in production mode, use the following command
```bash
docker compose up
```

5. Access the application
The application will be available at `http://localhost:3000`

# API Documentation

## Create a short URL
**POST /{short_url}**

### Body
```json
{
    "long_url": "https://example.com"
}
```


Where `{short_url}` is the short version of the URL.

## Response
### 200 Ok
```json
{
    "short_url": "{short_url}",
    "long_url": "https://example.com"

}
```

### 500 Internal Server Error
```json
{
    "error": "Internal Server Error"
}
```

## Redirect to the long URL
**GET /{short_url}**

Where `{short_url}` is the short version of the URL.

## Response
### 301 Moved Permanently
Redirects to the long URL

### 404 Not Found
```json
{
    "error": "URL not found"
}
```