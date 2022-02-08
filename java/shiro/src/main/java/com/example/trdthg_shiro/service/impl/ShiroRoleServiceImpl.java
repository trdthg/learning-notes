package com.example.trdthg_shiro.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.QueryWrapper;
import com.example.trdthg_shiro.entity.ShiroRole;
import com.example.trdthg_shiro.entity.ShiroUser;
import com.example.trdthg_shiro.entity.ShiroUserRole;
import com.example.trdthg_shiro.mapper.ShiroRoleMapper;
import com.example.trdthg_shiro.mapper.ShiroUserMapper;
import com.example.trdthg_shiro.mapper.ShiroUserRoleMapper;
import com.example.trdthg_shiro.service.ShiroRoleService;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.example.trdthg_shiro.service.ShiroUserRoleService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

/**
 * <p>
 *  服务实现类
 * </p>
 *
 * @author trdthg
 * @since 2021-05-29
 */
@Service
public class ShiroRoleServiceImpl extends ServiceImpl<ShiroRoleMapper, ShiroRole> implements ShiroRoleService {

    @Autowired
    ShiroRoleMapper shiroRoleMapper;

    @Override
    public List<String> findByUserId(Integer id) {
        return shiroRoleMapper.findByUserId(id);
    }
}
