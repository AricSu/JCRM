<template v-slot:searchForm>
  <div class="app-container">
    <!-- visible.sync用于显示隐藏弹框 -->
    <!-- 修改界面 -->
    <el-dialog :visible.sync="centerDialogVisible">
      <!-- 数据绑定在editForm -->
      <el-form :model="editForm">
        <el-form-item label="订单号">
          <el-input v-model="editForm.order_id" />
        </el-form-item>
        <el-form-item label="客户名称">
          <el-input v-model="editForm.customer_name" />
        </el-form-item>
        <el-form-item label="产品数量">
          <el-input v-model="editForm.product_number" />
        </el-form-item>
        <el-form-item label="订单状态">
          <select v-model="editForm.order_status">
            <option value="0">客户未下单</option>
            <option value="1">客户已下单</option>
            <option value="2">订单已完成</option>
          </select>
        </el-form-item>
        <el-form-item label="订单类别">
          <select v-model="editForm.is_profit">
            <option value="1">有利润</option>
            <option value="0">无利润</option>
          </select>
        </el-form-item>
        <el-form-item label="回款情况">
          <select v-model="editForm.order_fee_status">
            <option value="0">未回款</option>
            <option value="1">已回款</option>
          </select>
        </el-form-item>
        <el-form-item label="创建时间">
          <el-input v-model="editForm.create_time" />
        </el-form-item>
        <el-form-item label="完成时间">
          <el-input v-model="editForm.finish_time" />
        </el-form-item>
        <el-form-item label="地址">
          <el-input v-model="editForm.location" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="editForm.comments" />
        </el-form-item>
      </el-form>
      <div>

        <!-- 编辑页面的取消事件 -->
        <!-- closeDialog()绑定点击取消事件，点击之后弹窗页面不展示 -->
        <el-button @click="closeDialog()">取消</el-button>

        <!-- 编辑页面的确定事件 -->
        <el-button type="primary" @click="sumbitEditRow()">确定</el-button>
      </div>
    </el-dialog>
    <el-table
      v-loading="listLoading"
      :data="list"
      element-loading-text="Loading"
      border
      fit
      highlight-current-row
    >
      <el-table-column align="center" label="id" width="95">
        <template slot-scope="scope">
          {{ scope.row.order_id }}
        </template>
      </el-table-column>
      <el-table-column label="客户名称" width="150">
        <template slot-scope="scope">
          {{ scope.row.customer_name }}
        </template>
      </el-table-column>
      <el-table-column label="客户号" width="70">
        <template slot-scope="scope">
          {{ scope.row.customer_id }}
        </template>
      </el-table-column>
      <el-table-column label="产品数量" width="110" align="center">
        <template slot-scope="scope">
          <span>{{ scope.row.product_number }}</span>
        </template>
      </el-table-column>
      <el-table-column align="center" prop="created_at" label="订单创建时间" width="200">
        <template slot-scope="scope">
          <i class="el-icon-time" />
          <span>{{ scope.row.create_time }}</span>
        </template>
      </el-table-column>
      <el-table-column align="center" prop="created_at" label="订单完成时间" width="200">
        <template slot-scope="scope">
          <i class="el-icon-time" />
          <span>{{ scope.row.finish_time }}</span>
        </template>
      </el-table-column>
      <el-table-column label="订单状态" width="110" align="center">
        <template slot-scope="scope">
          <el-tag :type="scope.row.is_profit | statusFilter">{{ scope.row.is_profit | formatProfit }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="订单类别" width="110" align="center">
        <template slot-scope="scope">
          <el-tag :type="scope.row.order_status | statusFilter">{{ scope.row.order_status | formatOrderType }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="回款情况" width="110" align="center">
        <template slot-scope="scope">
          <el-tag :type="scope.row.order_fee_status | statusFilter">{{ scope.row.order_fee_status | formatFee }}</el-tag>
        </template>
      </el-table-column>
      <el-table-column label="目的地位置" width="110" align="center">
        <template slot-scope="scope">
          {{ scope.row.location }}
        </template>
      </el-table-column>
      <el-table-column label="编辑操作" width="110" align="center">
        <!-- 编辑按钮 -->
        <!-- scope.$index拿到每一行的index,
            scope.row拿到每一行的数据。 -->
        <el-button
          slot-scope="scope"
          type="primary"
          size="mini"
          @click="Edit(scope.$index, scope.row)"
        >编辑</el-button>
      </el-table-column>
      <el-table-column label="删除操作" width="110" align="center">
        <template slot-scope="scope">
          <el-popconfirm
            title="确定删除吗？"
            @onConfirm="Delete(scope.$index, scope.row)"
          >
            <!-- 删除按钮 -->
            <el-button slot="reference" size="mini" type="danger">删除</el-button>
          </el-popconfirm>
        </template>
      </el-table-column>
      <el-table-column label="备注" align="center">
        <template slot-scope="scope">
          {{ scope.row.comments }}
        </template>
      </el-table-column>

    </el-table>
  </div>
</template>

<script>
import { getList, updateOrder, deleteOrder } from '@/api/table'

export default {
  filters: {
    statusFilter(value) {
      const statusMap = {
        1: 'success',
        0: 'danger',
        2: 'info'
      }
      return statusMap[value]
    },
    formatProfit(status) {
      const statusMap = {
        0: '无利润',
        1: '有利润'
      }
      return statusMap[status]
    },
    formatOrderType(status) {
      const statusMap = {
        0: '客户未下单',
        1: '客户已下单',
        2: '订单已完成'
      }
      return statusMap[status]
    },
    formatFee(status) {
      const statusMap = {
        0: '未回款',
        1: '已回款'
      }
      return statusMap[status]
    }
  },
  data() {
    return {
      editForm: {},
      centerDialogVisible: false,
      searchForm: [],
      temp: {
        finish: []
      },
      list: null,
      listLoading: true,
      index: ''
    }
  },
  created() {
    this.fetchData()
  },
  methods: {
    fetchData() {
      this.listLoading = true
      getList().then(response => {
        this.list = response.data
        this.listLoading = false
      })
      console.log(this.list)
    },
    mergeSearchForm() {
      return {
        ...this.searchForm,
        status: this.temp.status.join(','),
        finish: this.temp.finish.join(',')
      }
    },
    // 修改按钮方法
    Edit(index, row) {
      // 修改页面打开
      this.centerDialogVisible = true

      // 重置对象
      this.editForm = Object.assign({}, row)
      this.index = index
    },

    //  修改页面确定按钮方法
    sumbitEditRow() {
      var editData = this.index
      this.list[editData].order_id = this.editForm.order_id
      this.list[editData].customer_name = this.editForm.customer_name
      this.list[editData].product_number = this.editForm.product_number
      this.list[editData].create_time = this.editForm.create_time
      this.list[editData].finish_time = this.editForm.finish_time
      this.list[editData].is_profit = this.editForm.is_profit
      this.list[editData].order_status = this.editForm.order_status
      this.list[editData].order_fee_status = this.editForm.order_fee_status
      this.list[editData].location = this.editForm.location
      this.list[editData].comments = this.editForm.comments
      updateOrder(this.list[editData]).then(() => {
        this.$router.push({ path: this.redirect || '/sale/selectorders' })
      })

      alert('修改成功！')
      // 修改好信息之后关闭修改页面
      this.centerDialogVisible = false
    },

    // 关闭所有弹框
    closeDialog() {
      // 弹窗页面不显示
      this.centerDialogVisible = false
      this.isAddMembers = false
    },

    // 删除按钮功能实现
    // confirm点击确定时触发
    Delete(index) {
      this.list.splice(index, 0)
      deleteOrder(this.list[index])
      alert('删除成功！')
      window.location.reload(true)
    }
  }
}
</script>
