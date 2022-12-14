<template>
  <div class="app-container">
    <el-form ref="form" :model="form" label-width="120px">
      <el-form-item label="唯一客户号">
        <el-input v-model="form.customer_id" />
      </el-form-item>
      <el-form-item label="客户名称">
        <el-input v-model="form.customer_name" />
      </el-form-item>
      <el-form-item label="快递商家">
        <el-input v-model="form.delivery_provider" />
      </el-form-item>
      <el-form-item label="产品数量">
        <el-input v-model="form.product_number" />
      </el-form-item>
      <el-form-item label="快递价格">
        <el-input v-model="form.order_price" />
      </el-form-item>
      <el-form-item label="快递类别">
        <el-select v-model="form.delivery_status" placeholder="请选择订单状态">
          <el-option label="快递未发出" value="0" />
          <el-option label="快递已发出" value="1" />
          <el-option label="快递已完成" value="2" />
        </el-select>
      </el-form-item>
      <el-form-item label="创建时间">
        <el-col :span="11">
          <el-time-picker v-model="form.create_date" type="fixed-time" placeholder="选择时间" value-format="yyyy-MM-dd HH:mm:ss" format="yyyy-MM-dd HH:mm:ss" style="width: 100%;" />
        </el-col>
      </el-form-item>
      <el-form-item label="收货地址">
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
import { addDelivery } from '@/api/delivery'
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
      addDelivery(this.form).then(() => {
        this.$router.push({ path: this.redirect || '/delivery/selectDeliveries' })
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

