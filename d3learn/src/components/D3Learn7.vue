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
    let data = [
      { platform: "Android", percentage: 40.11 },
      { platform: "Windows", percentage: 36.69 },
      { platform: "iOS", percentage: 13.06 },
    ];

    let svgWidth = 500,
      svgHeight = 300,
      radius = Math.min(svgWidth, svgHeight) / 2;
    let svg = d3
      .select("svg")
      .attr("width", svgWidth)
      .attr("height", svgHeight);

    //Create group element to hold pie chart
    let g = svg
      .append("g")
      .attr("transform", "translate(" + radius + "," + radius + ")");

    // d3.scaleOrdinal() 序数比例尺
    // schemeCategory10, 颜色比例尺
    // D3提供了一些颜色比例尺，10就是10种颜色，20就是20种：
    let color = d3.scaleOrdinal(d3.schemeCategory10);

    let pie = d3.pie().value((d) => d.percentage);

    let path = d3.arc().outerRadius(radius).innerRadius(0);

    let arc = g.selectAll("arc").data(pie(data)).enter().append("g");

    arc
      .append("path")
      .attr("d", path)
      .attr("fill", (d) => color(d.data.percentage));

    let label = d3.arc().outerRadius(radius).innerRadius(0);

    arc
      .append("text")
      .attr("transform", (d) => `translate(${label.centroid(d)})`)
      .attr("text-anchor", "middle")
      .text((d) => `${d.data.platform}:${d.data.percentage}%`);
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
