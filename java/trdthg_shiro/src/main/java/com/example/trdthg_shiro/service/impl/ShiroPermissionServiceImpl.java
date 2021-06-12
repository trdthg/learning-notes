package com.example.trdthg_shiro.service.impl;

import com.example.trdthg_shiro.entity.ShiroPermission;
import com.example.trdthg_shiro.mapper.ShiroPermissionMapper;
import com.example.trdthg_shiro.service.ShiroPermissionService;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
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
public class ShiroPermissionServiceImpl extends ServiceImpl<ShiroPermissionMapper, ShiroPermission> implements ShiroPermissionService {
    @Autowired
    ShiroPermissionMapper shiroPermissionMapper;

    @Override
    public List<String> findByUserId(Integer id) {
        return shiroPermissionMapper.findByUserId(id);
    }
}
