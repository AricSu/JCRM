<template>
  <div class="app-container">
    <el-form ref="form" :model="form" label-width="120px">
      <el-form-item label="唯一客户号">
        <el-input v-model="form.customer_id" />
      </el-form-item>
      <el-form-item label="客户名称">
        <el-input v-model="form.customer_name" />
      </el-form-item>
      <el-form-item label="产品数量">
        <el-input v-model="form.product_number" />
      </el-form-item>
      <el-form-item label="订单类别">
        <el-select v-model="form.is_profit" placeholder="请选择订单状态">
          <el-option label="有利润" value="1" />
          <el-option label="无利润" value="0" />
        </el-select>
      </el-form-item>
      <el-form-item label="创建时间">
        <el-col :span="11">
          <el-time-picker v-model="form.create_time" type="fixed-time" placeholder="选择时间" value-format="yyyy-MM-dd HH:mm:ss" format="yyyy-MM-dd HH:mm:ss" style="width: 100%;" />
        </el-col>
      </el-form-item>
      <el-form-item label="订单状态">
        <el-select v-model="form.order_status" placeholder="请选择订单状态">
          <el-option label="客户未下单" value="0" />
          <el-option label="客户已下单" value="1" />
        </el-select>
      </el-form-item>
      <el-form-item label="位置">
        <el-input v-model="form.location" type="textarea" />
      </el-form-item>
      <el-form-item label="备注">
        <el-input v-model="form.comments" type="textarea" />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSubmit">创建</el-button>
        <el-button @click="onCancel">取消</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script>
import { addList } from '@/api/table'
export default {
  data() {
    return {
      form: {
        customer_name: '',
        product_number: '',
        is_profit: '',
        create_time: '',
        order_status: '',
        location: '',
        comments: ''
      }
    }
  },
  methods: {
    onSubmit() {
      addList(this.form).then(() => {
        this.$router.push({ path: this.redirect || '/sale/selectorders' })
      })
      this.$message('submit!')
    },
    onCancel() {
      this.$message({
        message: 'cancel!',
        type: 'warning'
      })
    }
  }
}
</script>

  <style scoped>
  .line{
    text-align: center;
  }
  </style>

