<template>
  <h1>First heading</h1>
</template>

<script>
import * as d3 from "d3";

export default {
  name: "D3Learn1",
  props: {
    msg: String,
  },
  mounted: function () {
    d3.select();
    d3.selectAll();

    d3.select("h1")
      .style("color", "red")
      .attr("class", "heading")
      .text("Updated h1 tag");

    d3.select("body").append("p").text("First Paragraph");
    d3.select("body").append("p").text("Second Paragraph");
    d3.select("body").append("p").text("Third Paragraph");

    d3.selectAll("p").style("");
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
