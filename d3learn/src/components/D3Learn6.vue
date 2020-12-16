<template>
  <div>
    <h1>Bar Chart using D3.js</h1>
    <svg class="bar-chart"></svg>
  </div>
</template>

<script>
import * as d3 from "d3";

export default {
  name: "D3Learn4",
  props: {
    msg: String,
  },
  mounted: function () {
    let svgWidth = 600,
      svgHeight = 500;
    let svg = d3
      .select("svg")
      .attr("width", svgWidth)
      .attr("height", svgHeight)
      .attr("class", "svg-container");

    let line = svg
      .append("line")
      .attr("x1", 100)
      .attr("x2", 500)
      .attr("y1", 50)
      .attr("y2", 50)
      .attr("stroke", "red");

    let rect = svg
      .append("rect")
      .attr("x", 100)
      .attr("y", 100)
      .attr("width", 200)
      .attr("height", 100)
      .attr("fill", "#9B95FF");

    let circle = svg
      .append("circle")
      .attr("cx", 200)
      .attr("cy", 300)
      .attr("r", 80)
      .attr("fill", "#7CE8D5");
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
