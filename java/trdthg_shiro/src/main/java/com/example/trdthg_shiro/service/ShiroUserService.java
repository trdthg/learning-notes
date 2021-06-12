package com.example.trdthg_shiro.service;

import com.baomidou.mybatisplus.core.conditions.Wrapper;
import com.example.trdthg_shiro.entity.ShiroUser;
import com.baomidou.mybatisplus.extension.service.IService;
import org.springframework.stereotype.Repository;

import java.util.Map;

/**
 * <p>
 *  服务类
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
public interface ShiroUserService extends IService<ShiroUser> {

    ShiroUser findByUsername(String username);

    Map<String, String> saveToken(String username);

    Map<String, Object> refreshToken(String refreshToken);

    public String createAccessToken(String username);

    public String createRefreshToken(String username);

    public Map<String,Object> getInfoFromToken(String token);

//    boolean verifyToken(String token);

}

