<template v-slot:searchForm>
  <div class="app-container">
    <!-- visible.sync用于显示隐藏弹框 -->
    <!-- 修改界面 -->
    <el-dialog :visible.sync="centerDialogVisible">
      <!-- 数据绑定在editForm -->
      <el-form :model="editForm">
        <el-form-item label="快递编号">
          <el-input v-model="editForm.delivery_id" />
        </el-form-item>
        <el-form-item label="客户编号">
          <el-input v-model="editForm.customer_id" />
        </el-form-item>
        <el-form-item label="产品数量">
          <el-input v-model="editForm.product_number" />
        </el-form-item>
        <el-form-item label="快递状态">
          <select v-model="editForm.delivery_status">
            <option value="0">快递未发出</option>
            <option value="1">快递已发出</option>
            <option value="2">快递已完成</option>
          </select>
        </el-form-item>
        <el-form-item label="快递价格">
          <el-input v-model="editForm.order_price" />
        </el-form-item>
        <el-form-item label="漏气率">
          <el-input v-model="editForm.gas_leak_rate" />
        </el-form-item>
        <el-form-item label="创建时间">
          <el-input v-model="editForm.create_date" />
        </el-form-item>
        <el-form-item label="完成时间">
          <el-input v-model="editForm.finish_date" />
        </el-form-item>
        <el-form-item label="快递单号">
          <el-input v-model="editForm.delivery_number" />
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
      <el-table-column align="center" label="快递编号" width="95">
        <template slot-scope="scope">
          {{ scope.row.delivery_id }}
        </template>
      </el-table-column>
      <el-table-column label="客户编号" width="100" align="center">
        <template slot-scope="scope">
          {{ scope.row.customer_id }}
        </template>
      </el-table-column>
      <el-table-column label="产品数量" width="100" align="center">
        <template slot-scope="scope">
          {{ scope.row.product_number }}
        </template>
      </el-table-column>
      <el-table-column label="快递商家" width="110" align="center">
        <template slot-scope="scope">
          <span>{{ scope.row.delivery_provider }}</span>
        </template>
      </el-table-column>
      <el-table-column label="快递价格(元)" width="110" align="center">
        <template slot-scope="scope">
          <span>{{ scope.row.order_price }}</span>
        </template>
      </el-table-column>
      <el-table-column label="快递漏气率" width="110" align="center">
        <template slot-scope="scope">
          <span>{{ scope.row.gas_leak_rate }}</span>
        </template>
      </el-table-column>
      <el-table-column align="center" prop="created_at" label="订单创建时间" width="200">
        <template slot-scope="scope">
          <i class="el-icon-time" />
          <span>{{ scope.row.create_date }}</span>
        </template>
      </el-table-column>
      <el-table-column align="center" prop="created_at" label="订单完成时间" width="200">
        <template slot-scope="scope">
          <i class="el-icon-time" />
          <span>{{ scope.row.finish_date }}</span>
        </template>
      </el-table-column>
      <el-table-column label="目的地位置" width="200" align="center">
        <template slot-scope="scope">
          {{ scope.row.destination }}
        </template>
      </el-table-column>
      <el-table-column label="快递单号" width="200" align="center">
        <template slot-scope="scope">
          <span>{{ scope.row.delivery_number }}</span>
        </template>
      </el-table-column>
      <el-table-column label="快递状态" width="110" align="center">
        <template slot-scope="scope">
          <el-tag :type="scope.row.delivery_status | statusFilter">{{ scope.row.delivery_status | formatDelivery }}</el-tag>
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
import { getDeliveries, updateDelivery, deleteDelivery } from '@/api/delivery'

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
    formatDelivery(status) {
      const statusMap = {
        0: '未发出',
        1: '运送中',
        2: '已完成'
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
      getDeliveries().then(response => {
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
      this.list[editData].delivery_id = this.editForm.delivery_id
      this.list[editData].customer_id = this.editForm.customer_id
      this.list[editData].product_number = this.editForm.product_number
      this.list[editData].delivery_provider = this.editForm.delivery_provider
      this.list[editData].order_price = this.editForm.order_price
      this.list[editData].gas_leak_rate = this.editForm.gas_leak_rate
      this.list[editData].destination = this.editForm.destination
      this.list[editData].create_date = this.editForm.create_date
      this.list[editData].finish_date = this.editForm.finish_date
      this.list[editData].delivery_number = this.editForm.delivery_number
      this.list[editData].delivery_status = this.editForm.delivery_status
      this.list[editData].comments = this.editForm.comments
      updateDelivery(this.list[editData]).then(() => {
        this.$router.push({ path: this.redirect || '/delivery/selectDeliveries' })
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
      console.log(this.list[index])
      deleteDelivery(this.list[index])
      alert('删除成功！')
      window.location.reload(true)
    }
  }
}
</script>
