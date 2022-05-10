cp nginx.conf /etc/nginx/nginx.conf

php-fpm -D
nginx -g 'daemon off;'
