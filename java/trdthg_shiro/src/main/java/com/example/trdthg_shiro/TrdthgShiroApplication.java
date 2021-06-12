package com.example.trdthg_shiro;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@MapperScan("com.example.trdthg_shiro.mapper")
public class TrdthgShiroApplication {

    public static void main(String[] args) {
        SpringApplication.run(TrdthgShiroApplication.class, args);
    }

}
