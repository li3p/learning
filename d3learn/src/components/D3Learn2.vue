<template>
  <h1>First heading</h1>
</template>

<script>
import * as d3 from "d3";

export default {
  name: "D3Learn2",
  props: {
    msg: String,
  },
  mounted: function () {
    let dataset = [1, 2, 3, 4, 5];

    d3.select("body")
      .selectAll("p")
      .data(dataset)
      .enter()
      .append("p") // appends paragraph for each data element
      // .text("D3 is awesome!!");
      .text((d) => `${d} is awesome!!`);
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
