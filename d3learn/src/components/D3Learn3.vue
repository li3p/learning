<template>
  <div>
    <h1>Bar Chart using D3.js</h1>
    <svg class="bar-chart"></svg>
  </div>
</template>

<script>
import * as d3 from "d3";

export default {
  name: "D3Learn3",
  props: {
    msg: String,
  },

  mounted: function () {
    // 数据集
    let dataset = [80, 100, 56, 120, 180, 30, 40, 120, 160];

    // 定义svg图形宽高，以及柱状图间距
    let svgWidth = 500,
      svgHeight = 300,
      barPadding = 5;
    // 通过图形计算每个柱状宽度
    let barWidth = svgWidth / dataset.length;

    // 绘制图形
    let svg = d3
      .select("svg")
      .attr("width", svgWidth)
      .attr("height", svgHeight);

    // rect，长方形
    // 文档：http://www.w3school.com.cn/svg/svg_rect.asp

    let barChart = svg
      .selectAll("rect")
      .data(dataset) //绑定数组
      .enter() // 指定选择集的enter部分
      .append("rect") // 添加足够数量的矩形
      .attr("y", (d) => svgHeight - d) // d为数据集每一项的值, 取y坐标
      .attr("height", (d) => d) // 设定高度
      .attr("width", barWidth - barPadding) // 设定宽度
      .attr("transform", (d, i) => {
        let translate = [barWidth * i, 0];
        return "translate(" + translate + ")";
      }); // 实际是计算每一项值的x坐标

    let text = svg
      .selectAll("text")
      .data(dataset)
      .enter()
      .append("text")
      .text((d) => d)
      .attr("y", (d, _) => svgHeight - d - 2)
      .attr("x", (_, i) => barWidth * i)
      .attr("fill", "#A64C38");

    text.name;

    barChart.name;
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
