FROM node:16-alpine as deps

WORKDIR /app
COPY package*.json ./
RUN npm install --only=production

FROM node:16-alpine as build
WORKDIR /app
COPY package*.json ./
COPY tsconfig*.json ./
COPY src/ ./src
COPY --from=deps /app/node_modules ./node_modules
RUN npm i -g @nestjs/cli && npm run build

FROM node:16-alpine as runner
WORKDIR /app
ARG NODE_ENV=production

COPY --from=build app/dist ./dist
COPY --from=build app/node_modules ./node_modules

#CMD ["sleep", "1000"]
CMD ["node", "dist/main"]

