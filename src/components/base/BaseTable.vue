<script setup lang="ts" generic="T extends Record<string, unknown>">
defineProps<{
  data: T[]
  columns: { key: string; title: string }[]
}>()
</script>

<template>
  <table class="base-table">
    <thead>
      <tr>
        <th v-for="col in columns" :key="col.key">{{ col.title }}</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(row, i) in data" :key="i">
        <td v-for="col in columns" :key="col.key">{{ row[col.key] }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style lang="scss" scoped>
.base-table {
  width: 100%;
  border-collapse: collapse;

  th, td {
    padding: 12px 16px;
    text-align: left;
    border-bottom: 1px solid #eee;
  }

  th {
    font-weight: 600;
    color: #555;
    background: #fafafa;
  }

  tr:hover td {
    background: #f9f9f9;
  }
}
</style>