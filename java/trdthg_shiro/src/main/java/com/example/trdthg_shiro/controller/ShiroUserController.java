package com.example.trdthg_shiro.controller;


import com.example.trdthg_shiro.DTO.LoginDTO;
import com.example.trdthg_shiro.entity.ShiroUser;
import com.example.trdthg_shiro.service.ShiroUserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.validation.BindingResult;
import org.springframework.web.bind.annotation.*;

import org.springframework.stereotype.Controller;

import java.util.HashMap;
import java.util.Map;

@RestController
public class ShiroUserController {

    @Autowired
    ShiroUserService shiroUserService;

    @PostMapping("/login")
    public Map<String,String> login(@RequestBody LoginDTO loginDTO) {
        System.out.println("进入login");
        Map<String, String> resultMap = new HashMap<>();
        String username = loginDTO.getUsername();
        String password = loginDTO.getPassword();
        ShiroUser shiroUser = shiroUserService.findByUsername(username);
        if (shiroUser != null) {
            if (shiroUser.getPassword().equals(password)) {
                resultMap = shiroUserService.saveToken(username);
                resultMap.put("status", "200");
                resultMap.put("msg", "登录成功");
                return resultMap;
            } else {
                resultMap.put("msg", "密码错误");
            }
        } else {
            resultMap.put("msg", "没有该用户");
        }
        resultMap.put("status", "400");
        return resultMap;
    }

    @PostMapping("/refreshToken")
    public Map<String,Object> refreshToken(@RequestParam("refresh_token") String refreshToken) {
//        Map<String, String> resultMap = new HashMap<>();
        return shiroUserService.refreshToken(refreshToken);
    }

    @GetMapping("/index")
    public void index() {
        System.out.println("index");
        return;
    }

    @GetMapping("/select")
    public void select() {
        System.out.println("select");
        return;
    }

    @GetMapping("/vip")
    public void vip() {
        System.out.println("vip");
        return;
    }
}

