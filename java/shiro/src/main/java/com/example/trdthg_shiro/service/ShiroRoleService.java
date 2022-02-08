package com.example.trdthg_shiro.service;

import com.example.trdthg_shiro.entity.ShiroRole;
import com.baomidou.mybatisplus.extension.service.IService;
import com.example.trdthg_shiro.entity.ShiroUser;

import java.util.List;

/**
 * <p>
 * 服务类
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
public interface ShiroRoleService extends IService<ShiroRole> {

    List<String> findByUserId(Integer id);

}
