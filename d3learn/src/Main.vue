<template>
  <div id="app">
    <BarChart
      title="Bar Chart"
      xKey="name"
      yKey="amount"
      :data="barChartData"
    />

    <svg id="pie" width="800" height="800" />

    <!-- <article class="c_container c_small" id="wrap"></article> -->
  </div>
</template>

<script>
import * as d3 from "d3";
import BarChart from "./components/BarChart.vue";

export default {
  name: "App",
  components: {
    BarChart,
  },
  mounted: function () {
    let data = [
      { name: "Jim", votes: 12 },
      { name: "Sue", votes: 5 },
      { name: "Bob", votes: 21 },
      { name: "Ann", votes: 17 },
      { name: "Dan", votes: 3 },
    ];

    var pie = d3
      .pie()
      .value((d) => d.votes)
      .padAngle(0.025)
      .sort((a, b) => a.name < b.name)(data);

      pie
      

    var arcMkr = d3.arc().innerRadius(50).outerRadius(190).cornerRadius(6);
    var scC = d3.scaleOrdinal(d3.schemePastel2).domain(pie.map((d) => d.index));
    var g = d3
      .select("#pie")
      .append("g")
      .attr("transform", "translate(400,275)");

    g.selectAll("path")
      .data(pie)
      .enter()
      .append("path")
      .attr("d", arcMkr)
      .attr("fill", (d) => scC(d.index))
      .attr("stroke", "grey");

    g.selectAll("text")
      .data(pie)
      .enter()
      .append("text")
      .text((d) => d.data.name)
      .attr("x", (d) => arcMkr.innerRadius(85).centroid(d)[0])
      .attr("y", (d) => arcMkr.innerRadius(85).centroid(d)[1])
      .attr("font-family", "sans-serif")
      .attr("font-size", 14)
      .attr("text-anchor", "middle");

    // const wrap = d3.select("#wrap");

    // wrap
    //   .selectAll("section")
    //   .data(d3.schemePastel2)
    //   .enter()
    //   .append("section")
    //   .text((d) => d)
    //   .attr("class", "wrapper")
    //   .style("background", (d) => d3.color(d));
  },
  data: () => ({
    categorical: [
      { name: "schemeAccent", n: 8 },
      { name: "schemeDark2", n: 8 },
      { name: "schemePastel2", n: 8 },
      { name: "schemeSet2", n: 8 },
      { name: "schemeSet1", n: 9 },
      { name: "schemePastel1", n: 9 },
      { name: "schemeCategory10", n: 10 },
      { name: "schemeSet3", n: 12 },
      { name: "schemePaired", n: 12 },
      { name: "schemeCategory20", n: 20 },
      { name: "schemeCategory20b", n: 20 },
      { name: "schemeCategory20c", n: 20 },
    ],
    barChartData: [
      {
        name: "张三",
        amount: 25,
      },
      {
        name: "李四",
        amount: 40,
      },
      {
        name: "老王",
        amount: 15,
      },
      {
        name: "老赖",
        amount: 9,
      },
    ],
  }),
};
</script>

<style>
#app {
  font-family: "Open Sans", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #282f36;
  margin-top: 30px;
}
</style>
