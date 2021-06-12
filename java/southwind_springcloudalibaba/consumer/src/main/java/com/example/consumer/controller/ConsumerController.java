package com.example.consumer.controller;

import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.cloud.client.ServiceInstance;
import org.springframework.cloud.client.discovery.DiscoveryClient;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

@RestController
@Slf4j
public class ConsumerController {

    @Autowired
    private DiscoveryClient discoveryClient;
    @Autowired
    private RestTemplate restTemplate;  // 需要手动装载

    @GetMapping("/getInstances")
    public List<ServiceInstance> getInstances() {
        return this.discoveryClient.getInstances("provider");
    }

    // 原生RestTemplate
    @GetMapping("/getPort")
    public String getPort() {
        // 1. 获取随机一个provider实例
        List<ServiceInstance> providers = discoveryClient.getInstances("provider");
        int randomIndex = ThreadLocalRandom.current().nextInt(providers.size());
        ServiceInstance provider = providers.get(randomIndex);
        String uri = provider.getUri().toString();  // uri： ip + port
        String url = uri + "/getPort";
        // 2. 接口调用
        log.info("调用的端口是" + provider.getHost());
        String port = restTemplate.getForObject(url, String.class);
        return "调用Consumer，调用了端口为" + port + "的Provider的getPort";
    }

    // Ribbon负载均衡
    @GetMapping("/getPort2")
    public String getPort2() {
        // 默认轮询算法
        String port = restTemplate.getForObject("http://provider/getPort", String.class);
        return "调用Consumer，调用了端口为" + port + "的Provider的getPort";
    }
}
