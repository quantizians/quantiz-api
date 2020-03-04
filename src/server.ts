import { Application, Context } from "@curveball/core";

const app = new Application();

app.use((ctx: Context) => {
  ctx.status = 200;
  console.log(ctx);
  ctx.response.body = 'Hello world!';

});

console.log(`Server running at localhost:5555`);
app.listen(5555);