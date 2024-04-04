import("./pkg").then((module) => {
  console.log(module);
  let res = module.add_num(1, 3);
  console.log(res);
});
