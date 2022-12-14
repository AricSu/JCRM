import request from '@/utils/request'

export function getDeliveries() {
  return request({
    url: '/deliveries',
    method: 'get'
  })
}

export function addDelivery(data) {
  return request({
    url: '/delivery/add',
    method: 'post',
    data
  })
}

export function updateDelivery(data) {
  return request({
    url: '/delivery/add',
    method: 'post',
    data
  })
}

export function deleteDelivery(data) {
  return request({
    url: '/delivery/delete',
    method: 'post',
    data
  })
}
