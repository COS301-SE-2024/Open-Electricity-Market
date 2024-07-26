FROM node:18.18.0-alpine AS build
LABEL authors="Christiaan George Strydom"
WORKDIR /app/Frontend
COPY package*.json ./
RUN npm install
#RUN rm -rf build
COPY . .

EXPOSE 8080
#RUN npm run build
#FROM node:18.18.0-alpine AS run
#ENTRYPOINT [ "node","build" ]
CMD ["npm","run","build"]