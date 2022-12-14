import request from '@/utils/request'

export function getList(params) {
  return request({
    url: '/orders',
    method: 'get'
  })
}

export function addList(data) {
  return request({
    url: '/order/add',
    method: 'post',
    data
  })
}

export function updateOrder(data) {
  return request({
    url: '/order/add',
    method: 'post',
    data
  })
}

export function deleteOrder(data) {
  return request({
    url: '/order/delete',
    method: 'post',
    data
  })
}
