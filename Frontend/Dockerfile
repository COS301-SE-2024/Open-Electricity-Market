FROM node:18.18.0-alpine AS build
LABEL authors="Christiaan George Strydom"
WORKDIR /app/Frontend
COPY package*.json ./

RUN npm install
#RUN rm -rf build
COPY . .

RUN npm run build
#RUN npm prune --prod

#FROM node:18.18.0-alpine AS run
#WORKDIR /app/Frontend
#COPY --from=build /app/build ./build
#COPY --from=build /app/Frontend/package.json .
EXPOSE 8080
ENV NODE_ENV=production

#ENTRYPOINT [ "node","build" ]
CMD ["npm","run","test"]
#CMD ["node","test"]