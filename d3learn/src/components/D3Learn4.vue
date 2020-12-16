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
    let dataset = [1, 2, 3, 4, 5];

    let svgWidth = 500,
      svgHeight = 300,
      barPadding = 5;
    let barWidth = svgWidth / dataset.length;

    let svg = d3
      .select("svg")
      .attr("width", svgWidth)
      .attr("height", svgHeight);

    let yScale = d3
      .scaleLinear()
      .domain([0, d3.max(dataset)])
      .range([0, svgHeight]);

    let barChart = svg
      .selectAll("rect")
      .data(dataset)
      .enter()
      .append("rect")
      .attr("y", (d) => svgHeight - yScale(d))
      .attr("height", (d) => yScale(d))
      .attr("width", barWidth - barPadding)
      .attr("transform", (d, i) => {
        let translate = [barWidth * i, 0];
        return "translate(" + translate + ")";
      });

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
