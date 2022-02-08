package com.example.trdthg_shiro.service;

import com.example.trdthg_shiro.entity.ShiroUserRole;
import com.baomidou.mybatisplus.extension.service.IService;

import java.util.List;

/**
 * <p>
 * 服务类
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
public interface ShiroUserRoleService extends IService<ShiroUserRole> {

    public List<Integer> findByUserId(int id);

}
