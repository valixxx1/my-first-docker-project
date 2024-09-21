# What is it?
This is my first project with using Docker. It runs HTTP server with the text "Hello, world!"

# Installing
`docker build -t myserver https://github.com/valixxx1/my-first-docker-project.git#main`

Or this if you want to use your own index.html file:

```
git clone https://github.com/valixxx1/my-first-docker-project
cd my-first-docker-project
------- changing index.html file -------
docker build -t myserver .
```

# Run
`docker run -p 25565:25565 -d myserver`

Go to localhost:25565 in your browser and you will see the result.
