# Use the official Node.js latest image
FROM node:lts-buster-slim

# Create and set the working directory
WORKDIR /usr/src/app

# Copy package.json and package-lock.json before copying the rest of the files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy the rest of the application files
COPY . .

# Expose the port the app runs on
EXPOSE 3000

# Command to run the app
CMD ["npm", "run", "dev"]
