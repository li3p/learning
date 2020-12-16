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
    let data = [80, 100, 56, 120, 180, 30, 40, 120, 160];

    let svgWidth = 800,
      svgHeight = 400;

    let svg = d3
      .select("svg")
      .attr("width", svgWidth)
      .attr("height", svgHeight);

    // 首先是拿最大值构建x轴坐标
    let xScale = d3
      .scaleLinear()
      .domain([0, d3.max(data)])
      .range([0, svgWidth]);

    // 接下来是反转值，用作y轴坐标。
    let yScale = d3
      .scaleLinear()
      .domain([0, d3.max(data)])
      .range([svgHeight, 0]);

    // 横轴的API使用
    let x_axis = d3.axisBottom().scale(xScale);

    // 纵轴的API使用
    let y_axis = d3.axisLeft().scale(yScale);

    // 在svg中提供了如g元素这样的将多个元素组织在一起的元素。
    // 由g元素编组在一起的可以设置相同的颜色，可以进行坐标变换等，类似于Vue中的 <template>

    svg.append("g").attr("transform", "translate(50, 10)").call(y_axis);

    let xAxisTranslate = svgHeight - 20;

    svg
      .append("g")
      .attr("transform", "translate(50, " + xAxisTranslate + ")")
      .call(x_axis);
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
