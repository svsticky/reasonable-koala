FROM node:21-alpine as BUILDER
WORKDIR /app

COPY package.json .
COPY yarn.lock .
RUN yarn install

COPY public/ public/
COPY src/ src/
COPY index.html .
COPY vite.config.ts .
COPY tsconfig.json .
COPY tsconfig.node.json .

RUN yarn build

# production stage
FROM nginx:stable-alpine
COPY --from=BUILDER /app/dist /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/conf.d/default.conf 
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]